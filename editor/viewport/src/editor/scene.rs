use asset_id::TextureID;
use cgmath::{vec3, ElementWise, Vector3};
use formats::ascn::PointID;
use pin_vec::PinVec;

macro_rules! points {
    [$($p:literal),* $(,)?] => {[
        $(PointID::new($p).unwrap()),*
    ]};
}

macro_rules! face {
    ($t:literal: $p0:literal $p1:literal $p2:literal $p3:literal) => {
        Face {
            texture_id: TextureID($t),
            points: points![$p0, $p1, $p2, $p3],
            selected: false,
        }
    };
}

macro_rules! point {
    ($o:ident $e:ident [$x:literal $y:literal $z:literal]) => {
        Point {
            position: $o + $e.mul_element_wise(vec3($x as f32, $y as f32, $z as f32)),
            selected: false,
        }
    };
}

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
        self.redo_stack.clear();
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
                face!(0: 1 2 6 5),
                face!(0: 0 4 7 3),
                face!(0: 2 3 7 6),
                face!(0: 0 1 5 4),
                face!(0: 4 5 6 7),
                face!(0: 0 3 2 1),
            ],
            points: [
                point!(origin extent [0 0 0]),
                point!(origin extent [1 0 0]),
                point!(origin extent [1 1 0]),
                point!(origin extent [0 1 0]),
                point!(origin extent [0 0 1]),
                point!(origin extent [1 0 1]),
                point!(origin extent [1 1 1]),
                point!(origin extent [0 1 1]),
            ],
            selected: false,
        }
    }
}

pub struct Face {
    texture_id: TextureID,
    points: [PointID; 4],
    selected: bool,
}

pub struct Point {
    position: Vector3<f32>,
    selected: bool,
}
