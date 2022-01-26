use std::{collections::HashMap, rc::Rc};

use asset_id::TextureID;
use cgmath::{vec2, InnerSpace, Matrix4, Vector3};
use renderer::{
    data::{gizmo, line, solid},
    scene::{LineObject, SolidObject},
    Renderer,
};

use super::scene::{FaceID, PointID};

pub struct MeshGenInput<'a, I, S: 'a>
where
    I: Iterator<Item = &'a S>,
    S: DrawableSolid,
{
    pub renderer: &'a Renderer,
    pub mask: GraphicsMask,
    pub solids: I,
}

pub struct Graphics {
    pub solid_objects: Vec<SolidObject>,
    pub line_object: LineObject,
    pub point_gizmos: Rc<gizmo::Instances>,
}

pub trait DrawableSolid {
    fn selected(&self) -> bool;
    fn face(&self, face: FaceID) -> FaceData;
    fn point(&self, point: PointID) -> PointData;
}

pub struct FaceData {
    pub texture: TextureID,
    pub points: [PointID; 4],
    pub selected: bool,
}

pub struct PointData {
    pub position: Vector3<f32>,
    pub selected: bool,
}

pub enum GraphicsMask {
    Solids,
    Faces,
    Points,
}

pub fn mesh_gen<'a, I, S>(input: MeshGenInput<'a, I, S>, graphics: &mut Option<Graphics>)
where
    I: Iterator<Item = &'a S>,
    S: DrawableSolid + 'a,
{
    let transform = Rc::new(input.renderer.create_transform());

    let old_texture_ids = graphics.as_ref().map(|output| {
        output
            .solid_objects
            .iter()
            .map(|solid_object| solid_object.texture_id)
            .collect::<Vec<_>>()
    });

    let mut batches = HashMap::<TextureID, (Vec<solid::Vertex>, Vec<[u16; 3]>)>::new();
    let mut point_gizmos = Vec::new();
    let mut lines = Vec::new();

    let mut add_line = |solid: &S, a: usize, b: usize| {
        lines.push(line::Vertex {
            position: solid.point(PointID(a)).position,
            color: [0.0; 3],
        });
        lines.push(line::Vertex {
            position: solid.point(PointID(b)).position,
            color: [0.0; 3],
        });
    };

    for solid in input.solids {
        for face in 0..6 {
            let face = solid.face(FaceID(face));

            let (vertices, triangles) = batches.entry(face.texture).or_default();
            let t0 = vertices.len() as u16;

            triangles.push([t0, t0 + 1, t0 + 2]);
            triangles.push([t0, t0 + 2, t0 + 3]);

            let points = face.points.map(|point| solid.point(point));

            let normal = {
                let edge0 = points[1].position - points[0].position;
                let edge1 = points[3].position - points[0].position;
                edge0.cross(edge1).normalize()
            };

            for point in points {
                let position = point.position;
                let has_tint = match input.mask {
                    GraphicsMask::Solids => solid.selected(),
                    GraphicsMask::Faces => face.selected,
                    GraphicsMask::Points => false,
                };

                vertices.push(solid::Vertex {
                    position,
                    normal,
                    texcoord: if normal.x.abs() > normal.y.abs() {
                        if normal.x.abs() > normal.z.abs() {
                            vec2(position.y, position.z)
                        } else {
                            vec2(position.x, position.y)
                        }
                    } else if normal.y.abs() > normal.z.abs() {
                        vec2(position.x, position.z)
                    } else {
                        vec2(position.x, position.y)
                    } / 4.0,
                    tint: if has_tint {
                        [0.04, 0.36, 0.85, 0.5]
                    } else {
                        [0.0; 4]
                    },
                });
            }
        }

        for disp in [0, 4] {
            add_line(solid, disp, disp + 1);
            add_line(solid, disp + 1, disp + 2);
            add_line(solid, disp + 2, disp + 3);
            add_line(solid, disp + 3, disp);
        }

        for segment in 0..4 {
            add_line(solid, segment, segment + 4);
        }

        if matches!(input.mask, GraphicsMask::Points) {
            for point in 0..8 {
                let point = solid.point(PointID(point));
                point_gizmos.push(gizmo::Instance {
                    matrix: Matrix4::from_translation(point.position).into(),
                    color: if point.selected {
                        [0.04, 0.36, 0.85, 0.0]
                    } else {
                        [0.0; 4]
                    },
                });
            }
        }
    }

    if let Some(old_texture_ids) = old_texture_ids {
        for old_texture_id in old_texture_ids {
            batches.entry(old_texture_id).or_default();
        }
    }

    *graphics = Some(Graphics {
        solid_objects: batches
            .into_iter()
            .map(|(texture_id, (vertices, triangles))| SolidObject {
                texture_id,
                transform: transform.clone(),
                mesh: Rc::new(input.renderer.create_mesh(&vertices, &triangles)),
            })
            .collect(),
        line_object: LineObject {
            transform,
            lines: Rc::new(input.renderer.create_lines(&lines)),
        },
        point_gizmos: Rc::new(input.renderer.create_gizmo_instances(&point_gizmos)),
    });
}