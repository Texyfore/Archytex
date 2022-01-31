use std::rc::Rc;

use cgmath::vec3;
use renderer::{
    data::{grid, line},
    scene::{GridObject, Scene},
    Renderer,
};

pub struct Grid {
    step: i32,
    graphics: Vec<GridObject>,
    needs_regen: bool,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            step: 100,
            graphics: Vec::new(),
            needs_regen: true,
        }
    }
}

impl Grid {
    pub fn increase(&mut self) {
        self.step = (self.step * 10).clamp(1, 1000);
        self.needs_regen = true;
    }

    pub fn decrease(&mut self) {
        self.step = (self.step / 10).clamp(1, 1000);
        self.needs_regen = true;
    }

    pub fn set_step(&mut self, step: i32) {
        self.step = step.clamp(1, 1000);
    }

    pub fn step(&self) -> i32 {
        self.step
    }

    pub fn regen(&mut self, renderer: &Renderer) {
        if !self.needs_regen {
            return;
        }

        let mut lattice = Vec::new();

        let max = (self.step as f32 / 100.0) * 35.0;
        let min = -max;

        for sign in [-1.0, 1.0] {
            for i in 1..=35 {
                let dist = i as f32 * sign;

                lattice.push(line::Vertex {
                    position: vec3(dist, 0.0, min),
                    color: [1.0; 3],
                });

                lattice.push(line::Vertex {
                    position: vec3(dist, 0.0, max),
                    color: [1.0; 3],
                });

                lattice.push(line::Vertex {
                    position: vec3(min, 0.0, dist),
                    color: [1.0; 3],
                });

                lattice.push(line::Vertex {
                    position: vec3(max, 0.0, dist),
                    color: [1.0; 3],
                });
            }
        }

        lattice.push(line::Vertex {
            position: vec3(min, 0.0, 0.0),
            color: [1.0; 3],
        });

        lattice.push(line::Vertex {
            position: vec3(max, 0.0, 0.0),
            color: [1.0; 3],
        });

        lattice.push(line::Vertex {
            position: vec3(0.0, 0.0, min),
            color: [1.0; 3],
        });

        lattice.push(line::Vertex {
            position: vec3(0.0, 0.0, max),
            color: [1.0; 3],
        });

        self.graphics = vec![GridObject {
            lines: Rc::new(renderer.create_lines(&lattice)),
            info: Rc::new(renderer.create_grid_info_holder(&grid::Info {
                step: self.step,
                snap_flag: 0,
            })),
        }];
        self.needs_regen = false;
    }

    pub fn render(&self, scene: &mut Scene) {
        for grid_object in &self.graphics {
            scene.push_grid_object(grid_object.clone());
        }
    }
}
