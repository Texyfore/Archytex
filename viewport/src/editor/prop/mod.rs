use std::{cmp::Ordering, collections::HashMap, rc::Rc};

use cgmath::{Deg, ElementWise, InnerSpace, Matrix3, Matrix4, Transform, Vector3, Zero};

use crate::{
    input::InputMapper,
    math::{MinMax, Ray, SolidUtil},
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
}

struct Prop {
    id: PropID,
    transform: RTransform,
    location: Location,
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
                    selected: false,
                });

                self.rebuild = true;
            }
        }

        if state.input.is_active_once(Select) {
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
