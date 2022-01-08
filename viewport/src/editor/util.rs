use cgmath::{vec3, ElementWise, Matrix3, Vector3};

use crate::render::LineVertex;

pub fn line_cuboid(
    center: Vector3<f32>,
    half_extent: Vector3<f32>,
    rotation: Matrix3<f32>,
    inset: f32,
    color: [f32; 3],
) -> Vec<LineVertex> {
    let insets = [
        vec3(inset, inset, inset),
        vec3(-inset, inset, inset),
        vec3(-inset, inset, -inset),
        vec3(inset, inset, -inset),
        vec3(inset, -inset, inset),
        vec3(-inset, -inset, inset),
        vec3(-inset, -inset, -inset),
        vec3(inset, -inset, -inset),
    ];

    let extent = half_extent * 2.0;

    #[rustfmt::skip]
    let mut points = [
        vec3(-0.5, -0.5, -0.5),
        vec3( 0.5, -0.5, -0.5),
        vec3( 0.5, -0.5,  0.5),
        vec3(-0.5, -0.5,  0.5),
        vec3(-0.5,  0.5, -0.5),
        vec3( 0.5,  0.5, -0.5),
        vec3( 0.5,  0.5,  0.5),
        vec3(-0.5,  0.5,  0.5),
    ];

    for (i, p) in points.iter_mut().enumerate() {
        *p = center + rotation * (p.mul_element_wise(extent) + insets[i]);
    }

    let lines = [
        [0, 1],
        [1, 2],
        [2, 3],
        [3, 0],
        [4, 5],
        [5, 6],
        [6, 7],
        [7, 4],
        [0, 4],
        [1, 5],
        [2, 6],
        [3, 7],
    ];

    let mut vertices = Vec::new();

    for line in lines {
        for point in line {
            vertices.push(LineVertex {
                position: points[point].into(),
                color: [color[0], color[1], color[2], 1.0],
            });
        }
    }

    vertices
}
