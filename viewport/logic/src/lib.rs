mod input;
mod model;
mod camera;

use input::InputMapper;
use model::{Points, Polygon};
use tools::{
    app::{event::Event, input::ButtonKind, App, MainLoop},
    math::{InnerSpace, Mat4, SquareMatrix, Vec3, Vec4},
};

use crate::input::ElementKind;

pub struct Viewport {
    input_mapper: InputMapper,
    points: Points,
    poly_points: Vec<u16>,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            input_mapper: Default::default(),
            points: Default::default(),
            poly_points: Default::default(),
        }
    }
}

impl MainLoop for Viewport {
    fn process(&mut self, app: &mut App) {
        while let Some(event) = app.poll_event() {
            match event {
                Event::Initialized => {
                    self.input_mapper
                        .register_action("add_point", vec![ElementKind::Button(ButtonKind::Left)]);
                }
                Event::Resized(width, height) => {}
                Event::RawInput(input) => self.input_mapper.process_raw_input(input),
            };
        }

        if self.input_mapper.query_action_once("add_point") {
            let point = mouse_to_3d(self.input_mapper.query_mouse_pos());
            self.poly_points.push(self.points.insert(point));
        }

        let polygon = Polygon::new(self.poly_points.clone(), vec![]);
        polygon
            .gen_wireframe(&self.points, app.graphics())
            .draw(app.graphics(), Mat4::identity());
    }
}

fn mouse_to_3d(mouse: (f32, f32)) -> Vec3 {
    let (x, y) = mouse;
    let (x, y) = ((x / 1024.0 - 0.5) * 2.0, ((768.0 - y) / 768.0 - 0.5) * 2.0);

    let beg = unproject(Vec3::new(x, y, -1.0));
    let end = unproject(Vec3::new(x, y, 1.0));

    let dir = (end - beg).normalize();
    beg + dir * 5.0
}

fn unproject(point: Vec3) -> Vec3 {
    let mat = tools::math::perspective(tools::math::Deg(60.0), 1024.0 / 768.0, 0.1, 100.0);
    let point = mat.invert().unwrap() * Vec4::new(point.x, point.y, point.z, 1.0);
    Vec3::new(point.x / point.w, point.y / point.w, point.z / point.w)
}
