use tools::{
    gfx::{Graphics, LineMesh, LineVert},
    math::Vector3,
};

#[derive(Default)]
pub struct Points {
    storage: Vec<Option<Vector3<f32>>>,
    free_ids: Vec<usize>,
}

impl Points {
    pub fn insert(&mut self, point: Vector3<f32>) -> u16 {
        if let Some(id) = self.free_ids.pop() {
            self.storage[id] = Some(point);
            id as u16
        } else {
            self.storage.push(Some(point));
            self.storage.len() as u16 - 1
        }
    }

    pub fn remove(&mut self, id: u16) {
        let id = id as usize;
        if self.storage[id].is_none() {
            self.storage[id].take();
            self.free_ids.push(id);
        }
    }

    pub fn get(&self, id: u16) -> Option<&Vector3<f32>> {
        if let Some(point) = self.storage.get(id as usize) {
            point.as_ref()
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, id: u16) -> Option<&mut Vector3<f32>> {
        if let Some(point) = self.storage.get_mut(id as usize) {
            point.as_mut()
        } else {
            None
        }
    }
}

pub struct Polygon {
    outer: Vec<u16>,
    inner: Vec<Vec<u16>>,
}

impl Polygon {
    pub fn new(outer: Vec<u16>, inner: Vec<Vec<u16>>) -> Self {
        Self { outer, inner }
    }

    pub fn gen_wireframe(&self, points: &Points, gfx: &Graphics) -> LineMesh {
        let mut verts = Vec::new();

        verts.append(&mut to_verts(self.outer.clone(), points));
        for inner in self.inner.iter().cloned() {
            verts.append(&mut to_verts(inner, points));
        }

        LineMesh::new(gfx, &verts)
    }
}

fn to_verts(idx: Vec<u16>, points: &Points) -> Vec<LineVert> {
    loop_points(idx)
        .windows(2)
        .flatten()
        .map(|p| LineVert {
            pos: (*points.get(*p).unwrap()).into(),
            color: [1.0; 4],
        })
        .collect::<Vec<_>>()
}

fn loop_points(mut points: Vec<u16>) -> Vec<u16> {
    if !points.is_empty() {
        points.push(points[0]);
    }
    points
}
