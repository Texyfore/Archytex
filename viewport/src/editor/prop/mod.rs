use std::{cmp::Ordering, collections::HashMap, rc::Rc};

use cgmath::{Deg, ElementWise, InnerSpace, Matrix3, Matrix4, Transform, Vector3, Zero};

use crate::{
    input::InputMapper,
    math::{IntersectionPoint, MinMax, Plane, Ray, SolidUtil},
    render::{
        LineBatch, LineFactory, PropBank, PropID, Scene, SolidFactory, Transform as RTransform,
    },
    ring_vec::RingVec,
};

use super::{
    camera::WorldCamera,
    config::{MAX_PROPS, PRIMARY_COLOR},
    solid::SolidContainer,
    util,
    ActionBinding::*,
};

pub struct PropEditor {
    props: RingVec<Prop>,
    selection_lines: Rc<LineBatch>,
    rebuild: bool,
    move_op: Option<MoveOp>,
}

struct Prop {
    id: PropID,
    transform: RTransform,
    location: Location,
    previous_location: Location,
    selected: bool,
}

impl Prop {
    fn intersection_point(&self, prop_bank: &PropBank, ray: Ray) -> Option<Vector3<f32>> {
        if let Some((min, max)) = prop_bank.bounds(self.id) {
            let untransform = self.location.mat4().inverse_transform().unwrap();

            let ray_origin = (untransform * ray.origin.extend(1.0)).truncate();
            let ray_end = (untransform * ray.end.extend(1.0)).truncate();

            let ray_dir = ray_end - ray_origin;

            let t_min = (min - ray_origin).div_element_wise(ray_dir);
            let t_max = (max - ray_origin).div_element_wise(ray_dir);
            let t1 = t_min.min(t_max);
            let t2 = t_min.max(t_max);
            let near = t1.x.max(t1.y).max(t1.z);
            let far = t2.x.min(t2.y).min(t2.z);

            if near < far {
                return Some(ray.origin + (ray.end - ray.origin) * near);
            }
        }

        None
    }
}

struct MoveOp {
    plane: Plane,
    start: Vector3<f32>,
    end: Vector3<f32>,
}

#[derive(Clone, Copy)]
struct Location {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
}

impl Location {
    fn mat4(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position)
            * Matrix4::from_angle_x(Deg(self.rotation.x))
            * Matrix4::from_angle_y(Deg(self.rotation.y))
            * Matrix4::from_angle_z(Deg(self.rotation.z))
    }

    fn rot3(&self) -> Matrix3<f32> {
        Matrix3::from_angle_x(Deg(self.rotation.x))
            * Matrix3::from_angle_y(Deg(self.rotation.y))
            * Matrix3::from_angle_z(Deg(self.rotation.z))
    }
}

impl Default for Location {
    fn default() -> Self {
        Self {
            position: Vector3::zero(),
            rotation: Vector3::zero(),
        }
    }
}

impl PropEditor {
    pub fn new(line_factory: &LineFactory) -> Self {
        Self {
            props: RingVec::new(MAX_PROPS),
            selection_lines: line_factory.create(&[]),
            rebuild: false,
            move_op: None,
        }
    }

    pub fn process(&mut self, state: PropEditorState) {
        if state.input.is_active(Modifier) && state.input.is_active_once(AddProp) {
            if let Some(raycast) = state
                .solid_container
                .raycast(state.camera.screen_ray(state.input.mouse_pos()), true)
            {
                let location = Location {
                    position: raycast.point.snap(state.grid_length),
                    rotation: Vector3::zero(),
                };

                self.props.push(Prop {
                    id: 0,
                    transform: state.solid_factory.create_transform(),
                    location,
                    previous_location: location,
                    selected: false,
                });

                self.rebuild = true;
            }
        }

        if state.input.is_active_once(Select) && self.move_op.is_none() {
            if !state.input.is_active(EnableMultiSelect) {
                for (_, prop) in &mut self.props {
                    prop.selected = false;
                    self.rebuild = true;
                }
            }

            if let Some(prop) = self.raycast(
                state.solid_container,
                state.prop_bank,
                state.camera.screen_ray(state.input.mouse_pos()),
            ) {
                self.props.get_mut(prop).unwrap().selected = true;
                self.rebuild = true;
            }
        }

        let mut abort_move = false;
        if state.input.is_active_once(Move) {
            if self.move_op.is_some() {
                abort_move = true;
            } else {
                self.props
                    .iter_mut()
                    .filter(|(_, prop)| prop.selected)
                    .for_each(|(_, prop)| prop.previous_location = prop.location);

                let ray = state.camera.screen_ray(state.input.mouse_pos());
                let plane = self.move_plane(ray);
                if let Some(point) = ray.intersection_point(&plane) {
                    self.move_op = Some(MoveOp {
                        plane,
                        start: point.snap(state.grid_length),
                        end: point.snap(state.grid_length),
                    });
                }
            }
        }

        if self.move_op.is_some()
            && (state.input.is_active_once(AbortMove) || state.input.is_active_once(AbortMoveAlt))
        {
            abort_move = true;
        }

        if abort_move {
            self.props
                .iter_mut()
                .filter(|(_, prop)| prop.selected)
                .for_each(|(_, prop)| prop.location = prop.previous_location);
            self.move_op.take();
            self.rebuild = true;
        }

        if let Some(move_op) = self.move_op.as_mut() {
            let ray = state.camera.screen_ray(state.input.mouse_pos());
            if let Some(point) = ray.intersection_point(&move_op.plane) {
                move_op.end = (point + move_op.plane.normal * 0.01).snap(state.grid_length);
                let delta = move_op.end - move_op.start;
                self.props
                    .iter_mut()
                    .filter(|(_, prop)| prop.selected)
                    .for_each(|(_, prop)| {
                        prop.location.position = prop.previous_location.position + delta
                    });
                self.rebuild = true;
            }

            if state.input.is_active_once(ConfirmMove) {
                self.props.iter_mut().for_each(|(_, prop)| {
                    prop.selected = false;
                });
                self.move_op.take();
                self.rebuild = true;
            }
        }

        if state.input.is_active_once(DeleteSolid) {
            let selected: Vec<usize> = self
                .props
                .iter()
                .filter(|(_, prop)| prop.selected)
                .map(|(i, _)| i)
                .collect();
            for selected in selected {
                self.props.remove(selected);
            }
            self.rebuild = true;
        }

        if self.rebuild {
            let mut vertices = Vec::new();
            for (_, prop) in &mut self.props {
                prop.transform.set(prop.location.mat4());
                if prop.selected {
                    if let Some((min, max)) = state.prop_bank.bounds(prop.id) {
                        let rot3 = prop.location.rot3();
                        let (center, half_extent) = (prop.location.position, (max - min) * 0.5);
                        vertices.append(&mut util::line_cuboid(
                            center,
                            half_extent,
                            rot3,
                            0.0,
                            PRIMARY_COLOR,
                        ));
                    }
                }
            }
            self.selection_lines = state.line_factory.create(&vertices);
            self.rebuild = false;
        }
    }

    pub fn render(&self, scene: &mut Scene) {
        let mut batches: HashMap<PropID, Vec<RTransform>> = HashMap::new();

        for (_, prop) in &self.props {
            batches
                .entry(prop.id)
                .or_default()
                .push(prop.transform.clone());
        }

        scene.world_pass.props = batches.into_iter().collect();
        scene
            .world_pass
            .line_batches
            .push(self.selection_lines.clone());
    }

    fn raycast(
        &self,
        solid_container: &SolidContainer,
        prop_bank: &PropBank,
        ray: Ray,
    ) -> Option<usize> {
        let solid_point = solid_container
            .raycast(ray, false)
            .map(|raycast| raycast.point);

        let mut candidates = Vec::new();

        for (i, prop) in &self.props {
            if let Some(point) = prop.intersection_point(prop_bank, ray) {
                if let Some(solid_point) = solid_point {
                    if (solid_point - ray.origin).magnitude2() < (point - ray.origin).magnitude2() {
                        continue;
                    }
                }

                candidates.push((i, point));
            }
        }

        candidates.sort_unstable_by(|(_, a), (_, b)| {
            (a - ray.origin)
                .magnitude2()
                .partial_cmp(&(b - ray.origin).magnitude2())
                .unwrap_or(Ordering::Equal)
        });

        candidates.first().map(|(i, _)| *i)
    }

    fn move_plane(&self, ray: Ray) -> Plane {
        let mut center = Vector3::zero();
        let mut div = 0.0;

        for (_, prop) in &self.props {
            center += prop.location.position;
            div += 1.0;
        }

        center /= div;

        Plane {
            origin: center,
            normal: -(ray.vec().normalize()).cardinal(),
        }
    }
}

pub struct PropEditorState<'a> {
    pub input: &'a InputMapper,
    pub camera: &'a WorldCamera,
    pub solid_factory: &'a SolidFactory,
    pub line_factory: &'a LineFactory,
    pub prop_bank: &'a PropBank,
    pub solid_container: &'a SolidContainer,
    pub grid_length: f32,
}
