use std::{collections::HashMap, rc::Rc};

use asset_id::TextureID;
use cgmath::{vec2, InnerSpace, Matrix4};
use renderer::{
    data::{gizmo, line, solid},
    scene::{LineObject, SolidObject},
    Renderer,
};

use super::elements::{ElementKind, Solid};

pub struct MeshGenInput<'a, I>
where
    I: Iterator<Item = &'a Solid>,
{
    pub renderer: &'a Renderer,
    pub mask: ElementKind,
    pub solids: I,
}

pub struct Graphics {
    pub solid_objects: Vec<SolidObject>,
    pub line_object: LineObject,
    pub point_gizmos: Rc<gizmo::Instances>,
}

pub fn generate<'a, I>(input: MeshGenInput<'a, I>, graphics: &mut Option<Graphics>)
where
    I: Iterator<Item = &'a Solid>,
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
    let mut line_vertices = Vec::new();

    let mut add_line = |solid: &Solid, a: usize, b: usize| {
        line_vertices.push(line::Vertex {
            position: solid.points[a].meters(),
            color: [0.0; 3],
        });
        line_vertices.push(line::Vertex {
            position: solid.points[b].meters(),
            color: [0.0; 3],
        });
    };

    for solid in input.solids {
        for face in &solid.faces {
            let (vertices, triangles) = batches.entry(face.texture).or_default();
            let t0 = vertices.len() as u16;

            triangles.push([t0, t0 + 1, t0 + 2]);
            triangles.push([t0, t0 + 2, t0 + 3]);

            let points = face.points.map(|point| solid.points[point.0].meters());

            let normal = {
                let edge0 = points[1] - points[0];
                let edge1 = points[3] - points[0];
                edge0.cross(edge1).normalize()
            };

            for position in points {
                let has_tint = match input.mask {
                    ElementKind::Solid => solid.selected,
                    ElementKind::Face => face.selected,
                    ElementKind::Point => false,
                    ElementKind::Prop => false,
                };

                vertices.push(solid::Vertex {
                    position,
                    normal,
                    texcoord: if normal.x.abs() > normal.y.abs() {
                        if normal.x.abs() > normal.z.abs() {
                            vec2(position.z, position.y)
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

        if matches!(input.mask, ElementKind::Point) {
            for point in &solid.points {
                point_gizmos.push(gizmo::Instance {
                    matrix: Matrix4::from_translation(point.meters()).into(),
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
            lines: Rc::new(input.renderer.create_lines(&line_vertices)),
        },
        point_gizmos: Rc::new(input.renderer.create_gizmo_instances(&point_gizmos)),
    });
}
