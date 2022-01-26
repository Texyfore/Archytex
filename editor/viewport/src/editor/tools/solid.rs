use asset_id::TextureID;
use cgmath::{vec3, MetricSpace, Vector2, Vector3};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    editor::{
        graphics::{DrawableSolid, FaceData, GraphicsMask, PointData},
        scene::{Action, FaceID, PointID, RaycastEndpointKind, RaycastHit},
    },
    math::{MinMax, Snap},
};

use super::{generic, Context, Tool};

#[derive(Default)]
pub struct Hub {
    last_click_pos: Option<Vector2<f32>>,
}

impl Tool for Hub {
    fn process(&mut self, ctx: &mut Context) {
        if ctx.input().is_button_down_once(MouseButton::Left) {
            self.last_click_pos = Some(ctx.input().mouse_pos());
        }

        if let Some(last_click_pos) = self.last_click_pos {
            if ctx.input().mouse_pos().distance2(last_click_pos) > 100.0 {
                let ray = ctx.camera().screen_ray(last_click_pos);
                if let Some(hit) = ctx.scene().raycast(&ray) {
                    ctx.switch_to(Box::new(Add::new(
                        hit.endpoint.point + hit.endpoint.normal * 0.0001,
                    )));
                    return;
                }
            }
        }

        if ctx.input().was_button_down_once(MouseButton::Left) {
            self.last_click_pos = None;
            ctx.switch_to(Box::new(generic::Select::<SelectProvider>::default()));
        }

        if ctx.input().is_key_down_once(VirtualKeyCode::Delete) {
            ctx.switch_to(Box::new(generic::Delete::<DeleteProvider>::default()));
        }

        self.process_undo_redo(ctx);
        self.process_camera(ctx);
    }

    fn graphics_mask(&self) -> GraphicsMask {
        GraphicsMask::Solids
    }
}

#[derive(Default)]
struct SelectProvider;

impl generic::SelectProvider for SelectProvider {
    fn deselect_action() -> Action {
        Action::DeselectSolids
    }

    fn select_action(hit: RaycastHit) -> Option<Action> {
        if let RaycastEndpointKind::Face { solid_id, .. } = hit.endpoint.kind {
            Some(Action::SelectSolids(vec![solid_id]))
        } else {
            None
        }
    }

    fn parent_tool() -> Box<dyn Tool> {
        Box::new(Hub::default())
    }

    fn graphics_mask() -> GraphicsMask {
        GraphicsMask::Solids
    }
}

#[derive(Default)]
struct DeleteProvider;

impl generic::DeleteProvider for DeleteProvider {
    fn action() -> Action {
        Action::RemoveSelectedSolids
    }

    fn parent_tool() -> Box<dyn Tool> {
        Box::new(Hub::default())
    }

    fn graphics_mask() -> GraphicsMask {
        GraphicsMask::Solids
    }
}

struct Add {
    start: Vector3<f32>,
}

impl Add {
    fn new(start: Vector3<f32>) -> Self {
        Self { start }
    }
}

impl Tool for Add {
    fn process(&mut self, ctx: &mut Context) {
        let mouse_pos = ctx.input().mouse_pos();
        let ray = ctx.camera().screen_ray(mouse_pos);

        if let Some(hit) = ctx.scene().raycast(&ray) {
            let end = hit.endpoint.point + hit.endpoint.normal * 0.0001;

            let start = self.start.snap(100);
            let end = end.snap(100);

            let min = start.min(end);
            let max = start.max(end).map(|e| e + 100);
        }

        if ctx.input().was_button_down_once(MouseButton::Left) {
            ctx.switch_to(Box::new(Hub::default()));
        }
    }

    fn graphics_mask(&self) -> GraphicsMask {
        GraphicsMask::Solids
    }
}

struct DummySolid {
    faces: [FaceData; 6],
}

impl DummySolid {
    fn new(min: Vector3<i32>, max: Vector3<i32>) -> Self {
        Self {
            faces: [
                [1, 2, 6, 5],
                [0, 4, 7, 3],
                [2, 3, 7, 6],
                [0, 1, 5, 4],
                [4, 5, 6, 7],
                [0, 3, 2, 1],
            ]
            .map(|points| FaceData {
                points: points.map(|point| PointID(point)),
                texture: TextureID(0),
                selected: false,
            }),
        }
    }
}

impl DrawableSolid for DummySolid {
    fn selected(&self) -> bool {
        false
    }

    fn face(&self, face: FaceID) -> FaceData {
        self.faces[face.0]
    }

    fn point(&self, point: PointID) -> PointData {
        todo!()
    }
}
