mod camera;

use std::rc::Rc;

use anyhow::Result;
use renderer::{
    data::{GizmoInstance, GizmoMesh},
    scene::{GizmoObject, Scene},
    Renderer,
};
use tk3d::math::{perspective, vec3, Deg, Matrix4, SquareMatrix};

use self::camera::Camera;

pub struct Editor {
    camera: Camera,
    gizmo_mesh: Rc<GizmoMesh>,
}

impl Editor {
    pub fn new(renderer: &Renderer) -> Self {
        Self {
            camera: Camera::default(),
            gizmo_mesh: Rc::new(renderer.create_gizmo_mesh(
                &tk3d::agzm::GizmoMesh::decode(include_bytes!("gizmo.agzm")).unwrap(),
            )),
        }
    }

    pub fn process(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn render(&self, renderer: &Renderer) -> Result<()> {
        let mut scene = Scene::default();

        scene.set_camera_matrix(self.camera.matrix());
        scene.push_gizmo_object(GizmoObject {
            mesh: self.gizmo_mesh.clone(),
            instances: Rc::new(
                renderer
                    .create_gizmo_instances(&[GizmoInstance::new(Matrix4::identity(), [1.0; 3])]),
            ),
        });

        renderer.render(&scene)?;
        Ok(())
    }

    pub fn window_resized(&mut self, width: u32, height: u32) {
        self.camera.recreate_projection(width, height);
    }
}
