use std::{collections::HashMap, rc::Rc};

use cgmath::{Deg, Matrix3, Matrix4, Vector3, Zero};

use crate::{
    input::InputMapper,
    math::{Ray, SolidUtil},
    render::{LineBatch, LineFactory, PropBank, PropID, Scene, SolidFactory, Transform},
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
    transform: Transform,
    location: Location,
    selected: bool,
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
                .raycast(state.camera.screen_ray(state.input.mouse_pos()))
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

        // if state.input.is_active_once(Select) {
        //     if !state.input.is_active(EnableMultiSelect) {
        //         for (_, prop) in &mut self.props {
        //             prop.selected = false;
        //             self.rebuild = true;
        //         }
        //     }
        // }

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
                            -0.1,
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
        let mut batches: HashMap<PropID, Vec<Transform>> = HashMap::new();

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

    fn _raycast(
        &self,
        _solid_container: &SolidContainer,
        _prop_bank: &PropBank,
        _ray: Ray,
    ) -> Option<usize> {
        todo!()
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
