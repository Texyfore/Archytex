use asset::GizmoID;
use cgmath::{Matrix4, Vector3};

use crate::{
    graphics::{structures::GizmoInstance, Canvas, GizmoGroup, GizmoInstances, Graphics, Share},
    logic::editor::common::Axis,
};

use super::Selection;

pub struct ArrowGraphics {
    arrows: GizmoInstances,
    planes: GizmoInstances,
    sphere: GizmoInstances,
}

impl ArrowGraphics {
    pub fn new_empty(graphics: &Graphics) -> Self {
        Self {
            arrows: graphics.create_gizmo_instances(3),
            planes: graphics.create_gizmo_instances(3),
            sphere: graphics.create_gizmo_instances(1),
        }
    }

    pub fn modify(
        &self,
        graphics: &Graphics,
        position: Vector3<f32>,
        selected: Option<Selection>,
        pressed: bool,
    ) {
        let translation = Matrix4::from_translation(position);

        let arrow_instances = Axis::all()
            .into_iter()
            .map(|axis| {
                let mut color = axis.color();
                let mut scale = 15.0;

                if let Some(Selection::Axis(selected)) = selected {
                    if axis == selected {
                        if pressed {
                            color = [1.0; 3];
                            scale = 18.0;
                        } else {
                            color[0] += 0.1;
                            color[1] += 0.1;
                            color[1] += 0.1;
                            scale = 16.0;
                        }
                    }
                }

                GizmoInstance {
                    matrix: translation * axis.rotation_from_y() * Matrix4::from_scale(scale),
                    color,
                }
            })
            .collect::<Vec<_>>();

        let plane_instances = Axis::all()
            .into_iter()
            .map(|axis| {
                let mut color = axis.color();
                let mut scale = 15.0;

                if let Some(Selection::Plane(selected)) = selected {
                    if axis == selected {
                        if pressed {
                            color = [1.0; 3];
                            scale = 18.0;
                        } else {
                            color[0] += 0.1;
                            color[1] += 0.1;
                            color[1] += 0.1;
                            scale = 16.0;
                        }
                    }
                }

                GizmoInstance {
                    matrix: translation * axis.plane_rotation_from_y() * Matrix4::from_scale(scale),
                    color,
                }
            })
            .collect::<Vec<_>>();

        graphics.write_gizmo_instances(&self.arrows, &arrow_instances);
        graphics.write_gizmo_instances(&self.planes, &plane_instances);

        graphics.write_gizmo_instances(
            &self.sphere,
            &[GizmoInstance {
                matrix: Matrix4::from_translation(position),
                color: [1.0; 3],
            }],
        );
    }

    pub fn render(&self, canvas: &mut Canvas) {
        canvas.draw_gizmos_no_depth(GizmoGroup {
            gizmo: GizmoID(1),
            instances: self.arrows.share(),
        });

        canvas.draw_gizmos_no_depth(GizmoGroup {
            gizmo: GizmoID(2),
            instances: self.planes.share(),
        });

        canvas.draw_gizmos_no_depth(GizmoGroup {
            gizmo: GizmoID(0),
            instances: self.sphere.share(),
        });
    }
}

pub struct ArcGraphics {
    arcs: GizmoInstances,
    sphere: GizmoInstances,
}

impl ArcGraphics {
    pub fn new(graphics: &Graphics) -> Self {
        Self {
            arcs: graphics.create_gizmo_instances(3),
            sphere: graphics.create_gizmo_instances(1),
        }
    }

    pub fn modify(
        &self,
        graphics: &Graphics,
        position: Vector3<f32>,
        selected: Option<Axis>,
        pressed: bool,
    ) {
        let translation = Matrix4::from_translation(position);

        let arc_instances = Axis::all()
            .into_iter()
            .map(|axis| {
                let mut color = axis.color();
                let mut scale = 15.0;

                if let Some(selected) = selected {
                    if axis == selected {
                        if pressed {
                            color = [1.0; 3];
                            scale = 18.0;
                        } else {
                            color[0] += 0.1;
                            color[1] += 0.1;
                            color[1] += 0.1;
                            scale = 16.0;
                        }
                    }
                }

                GizmoInstance {
                    matrix: translation * axis.rotation_from_y() * Matrix4::from_scale(scale),
                    color,
                }
            })
            .collect::<Vec<_>>();

        graphics.write_gizmo_instances(&self.arcs, &arc_instances);
        graphics.write_gizmo_instances(
            &self.sphere,
            &[GizmoInstance {
                matrix: translation,
                color: [1.0; 3],
            }],
        );
    }

    pub fn render(&self, canvas: &mut Canvas) {
        canvas.draw_gizmos_no_depth(GizmoGroup {
            gizmo: GizmoID(3),
            instances: self.arcs.share(),
        });

        canvas.draw_gizmos_no_depth(GizmoGroup {
            gizmo: GizmoID(0),
            instances: self.sphere.share(),
        });
    }
}
