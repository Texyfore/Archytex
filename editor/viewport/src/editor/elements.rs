use asset_id::TextureID;
use cgmath::{vec3, ElementWise, Vector3, Zero};

macro_rules! points {
    [$($p:literal),* $(,)?] => {[
        $(PointID($p)),*
    ]};
}

macro_rules! face {
    ($t:literal: $p0:literal $p1:literal $p2:literal $p3:literal) => {
        Face {
            texture: TextureID($t),
            points: points![$p0, $p1, $p2, $p3],
            selected: false,
        }
    };
}

macro_rules! point {
    ($o:ident $e:ident [$x:literal $y:literal $z:literal]) => {
        Point {
            position: $o + $e.mul_element_wise(vec3($x, $y, $z)),
            selected: false,
        }
    };
}

macro_rules! entity_id {
    ($name:ident, $ty:ty) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(pub $ty);
    };
}

entity_id!(SolidID, u32);
entity_id!(FaceID, usize);
entity_id!(PointID, usize);
entity_id!(PropID, u32);

#[derive(Clone, Copy)]
pub enum ElementKind {
    Solid,
    Face,
    Point,
    Prop,
}

#[derive(Clone)]
pub struct Solid {
    pub faces: [Face; 6],
    pub points: [Point; 8],
    pub selected: bool,
}

impl Solid {
    pub fn new(origin: Vector3<i32>, extent: Vector3<i32>) -> Self {
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

impl Movable for Solid {
    fn center(&self) -> Vector3<f32> {
        let mut center = Vector3::zero();

        for point in &self.points {
            center += point.meters();
        }

        center / self.points.len() as f32
    }

    fn displace(&mut self, mask: ElementKind, delta: Vector3<i32>) -> bool {
        let mut modified = false;

        match mask {
            ElementKind::Solid => {
                for point in &mut self.points {
                    point.position += delta;
                    modified = true;
                }
            }
            ElementKind::Face => {
                let mut mod_arr = [false; 8];
                for face in self.faces.iter().filter(|face| face.selected) {
                    for point in face.points {
                        let index = point.0;
                        if !mod_arr[index] {
                            let point = &mut self.points[point.0];
                            point.position += delta;
                            mod_arr[index] = true;
                            modified = true;
                        }
                    }
                }
            }
            ElementKind::Point => {
                for point in self.points.iter_mut().filter(|point| point.selected) {
                    point.position += delta;
                    modified = true;
                }
            }
            ElementKind::Prop => {}
        }

        modified
    }
}

#[derive(Clone)]
pub struct Face {
    pub texture: TextureID,
    pub points: [PointID; 4],
    pub selected: bool,
}

#[derive(Clone)]
pub struct Point {
    pub position: Vector3<i32>,
    pub selected: bool,
}

impl Point {
    pub fn meters(&self) -> Vector3<f32> {
        self.position.map(|e| e as f32 * 0.01)
    }
}

pub trait Movable {
    fn center(&self) -> Vector3<f32>;
    fn displace(&mut self, mask: ElementKind, delta: Vector3<i32>) -> bool;
}
