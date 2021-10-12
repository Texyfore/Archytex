mod input;

use input::InputMapper;
use tools::{
    app::{event::Event, input::ButtonKind, App, MainLoop},
    gfx::{LineMesh, LineVert},
    math::{InnerSpace, Mat4, SquareMatrix, Vec3, Vec4},
};

use crate::input::ElementKind;

pub struct Viewport {
    input_mapper: InputMapper,
    points: Vec<Vec3>,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            input_mapper: Default::default(),
            points: Default::default(),
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
                Event::RawInput(input) => self.input_mapper.process_raw_input(input),
            };
        }

        if self.input_mapper.query_action_once("add_point") {
            let projection =
                tools::math::perspective(tools::math::Deg(60.0), 1024.0 / 768.0, 0.1, 100.0);

            let (x, y) = self.input_mapper.query_mouse_pos();
            let (x, y) = ((x / 1024.0 - 0.5) * 2.0, ((768.0 - y) / 768.0 - 0.5) * 2.0);

            let beg = unproject(Vec3::new(x, y, -1.0), projection);
            let end = unproject(Vec3::new(x, y, 1.0), projection);

            let dir = (end - beg).normalize();
            let point = beg + dir * 5.0;
            self.points.push(point);
        }

        if !self.points.is_empty() {
            let points = {
                let mut vec = self.points.clone();
                vec.push(self.points[0]);
                vec
            };

            LineMesh::new(
                app.graphics(),
                &points
                    .windows(2)
                    .flatten()
                    .map(|p| LineVert {
                        pos: [p.x, p.y, p.z],
                    })
                    .collect::<Vec<_>>(),
            )
            .draw(app.graphics(), Mat4::identity());
        }
    }
}

fn unproject(point: Vec3, mat: Mat4) -> Vec3 {
    let point = mat.invert().unwrap() * Vec4::new(point.x, point.y, point.z, 1.0);
    Vec3::new(point.x / point.w, point.y / point.w, point.z / point.w)
}
