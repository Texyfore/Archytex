use std::collections::HashMap;

use cgmath::{Deg, Matrix4, Vector3, Zero};

use crate::{
    input::InputMapper,
    render::{PropID, Scene, SolidFactory, Transform},
    ring_vec::RingVec,
};

use super::{config::MAX_PROPS, ActionBinding::*};

pub struct PropEditor {
    props: RingVec<Prop>,
}

struct Prop {
    id: PropID,
    transform: Transform,
    location: Location,
    previous_location: Location,
}

struct Location {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            position: Vector3::zero(),
            rotation: Vector3::zero(),
        }
    }
}

impl Default for PropEditor {
    fn default() -> Self {
        Self {
            props: RingVec::new(MAX_PROPS),
        }
    }
}

impl PropEditor {
    pub fn process(&mut self, state: PropEditorState) {
        if state.input.is_active_once(AddSolid) {
            self.props.push(Prop {
                id: 0,
                transform: state.solid_factory.create_transform(),
                location: Default::default(),
                previous_location: Default::default(),
            });
        }

        if self.props.has_element_at(0) {
            self.props.get_mut(0).unwrap().location.rotation.y += 5.0;
        }

        for (_, prop) in &mut self.props {
            prop.transform.set(
                Matrix4::from_angle_z(Deg(prop.location.rotation.z))
                    * Matrix4::from_angle_y(Deg(prop.location.rotation.y))
                    * Matrix4::from_angle_x(Deg(prop.location.rotation.x))
                    * Matrix4::from_translation(prop.location.position),
            );
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
    }
}

pub struct PropEditorState<'a> {
    pub input: &'a InputMapper,
    pub solid_factory: &'a SolidFactory,
}
