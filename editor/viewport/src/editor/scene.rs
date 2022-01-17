use asset_id::TextureID;
use cgmath::{vec3, ElementWise, Vector3};
use pin_vec::PinVec;

#[derive(Default)]
pub struct Scene {
    solids: PinVec<Solid>,
    undo_stack: Vec<Action>,
    redo_stack: Vec<Action>,
}

impl Scene {
    pub fn act(&mut self, action: Action) {
        let inverse = self.execute_action(action);
        self.undo_stack.push(inverse);
    }

    pub fn undo(&mut self) {
        if let Some(action) = self.undo_stack.pop() {
            let inverse = self.execute_action(action);
            self.redo_stack.push(inverse);
        }
    }

    pub fn redo(&mut self) {
        if let Some(action) = self.redo_stack.pop() {
            let inverse = self.execute_action(action);
            self.undo_stack.push(inverse);
        }
    }

    fn execute_action(&mut self, action: Action) -> Action {
        match action {
            Action::AddSolid(solid) => Action::RemoveSolid(self.solids.push(solid)),
            Action::RemoveSolid(index) => Action::AddSolid(self.solids.remove(index).unwrap()),
        }
    }
}

impl<'a> mesh_gen::Model<'a, Solid, Face, Point> for Scene {
    fn solids(&self) -> Vec<&Solid> {
        self.solids.iter().collect()
    }
}

pub enum Action {
    AddSolid(Solid),
    RemoveSolid(usize),
}

pub struct Solid {
    faces: [Face; 6],
    points: [Point; 8],
    selected: bool,
}

impl Solid {
    pub fn new(origin: Vector3<f32>, extent: Vector3<f32>) -> Self {
        Self {
            faces: [
                Face {
                    texture_id: TextureID(0),
                    points: [1, 2, 6, 5],
                    selected: false,
                },
                Face {
                    texture_id: TextureID(0),
                    points: [0, 4, 7, 3],
                    selected: false,
                },
                Face {
                    texture_id: TextureID(0),
                    points: [2, 3, 7, 6],
                    selected: false,
                },
                Face {
                    texture_id: TextureID(0),
                    points: [0, 1, 5, 4],
                    selected: false,
                },
                Face {
                    texture_id: TextureID(0),
                    points: [4, 5, 6, 7],
                    selected: false,
                },
                Face {
                    texture_id: TextureID(0),
                    points: [0, 3, 2, 1],
                    selected: false,
                },
            ],
            points: [
                Point {
                    position: origin + extent.mul_element_wise(vec3(0.0, 0.0, 0.0)),
                    selected: false,
                },
                Point {
                    position: origin + extent.mul_element_wise(vec3(1.0, 0.0, 0.0)),
                    selected: false,
                },
                Point {
                    position: origin + extent.mul_element_wise(vec3(1.0, 1.0, 0.0)),
                    selected: false,
                },
                Point {
                    position: origin + extent.mul_element_wise(vec3(0.0, 1.0, 0.0)),
                    selected: false,
                },
                Point {
                    position: origin + extent.mul_element_wise(vec3(0.0, 0.0, 1.0)),
                    selected: false,
                },
                Point {
                    position: origin + extent.mul_element_wise(vec3(1.0, 0.0, 1.0)),
                    selected: false,
                },
                Point {
                    position: origin + extent.mul_element_wise(vec3(1.0, 1.0, 1.0)),
                    selected: false,
                },
                Point {
                    position: origin + extent.mul_element_wise(vec3(0.0, 1.0, 1.0)),
                    selected: false,
                },
            ],
            selected: false,
        }
    }
}

impl mesh_gen::Solid<Face, Point> for Solid {
    fn faces(&self) -> &[Face; 6] {
        &self.faces
    }

    fn points(&self) -> &[Point; 8] {
        &self.points
    }
}

pub struct Face {
    texture_id: TextureID,
    points: [usize; 4],
    selected: bool,
}

impl mesh_gen::Face for Face {
    fn texture_id(&self) -> TextureID {
        self.texture_id
    }

    fn points(&self) -> &[usize; 4] {
        &self.points
    }
}

pub struct Point {
    position: Vector3<f32>,
    selected: bool,
}

impl mesh_gen::Point for Point {
    fn position(&self) -> Vector3<f32> {
        self.position
    }
}
