use std::rc::Rc;

use glow::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

pub struct Context {
    pub(super) gl: glow::Context,
    pub(super) program: WebProgramKey,
}

impl Context {
    pub fn new(canvas: &HtmlCanvasElement) -> Rc<Self> {
        let gl = glow::Context::from_webgl2_context(
            canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into::<WebGl2RenderingContext>()
                .unwrap(),
        );

        let program = unsafe {
            let program = gl.create_program().expect("Couldn't create mesh pipeline");

            {
                let vertex_shader = compile_shader(
                    &gl,
                    "mesh.vert",
                    include_str!("shaders/mesh.vert"),
                    VERTEX_SHADER,
                );

                let fragment_shader = compile_shader(
                    &gl,
                    "mesh.frag",
                    include_str!("shaders/mesh.frag"),
                    FRAGMENT_SHADER,
                );

                gl.attach_shader(program, vertex_shader);
                gl.attach_shader(program, fragment_shader);
                gl.link_program(program);
                gl.detach_shader(program, fragment_shader);
                gl.detach_shader(program, vertex_shader);

                gl.delete_shader(vertex_shader);
                gl.delete_shader(fragment_shader);
            }

            let info = gl.get_program_info_log(program);
            if !info.is_empty() {
                panic!("Link error in gl program: {}", info);
            }

            program
        };

        Rc::new(Self { gl, program })
    }
}

unsafe fn compile_shader(context: &glow::Context, name: &str, src: &str, ty: u32) -> WebShaderKey {
    let shader = context.create_shader(ty).expect("Couldn't create shader");

    context.shader_source(shader, src);
    context.compile_shader(shader);

    let info = context.get_shader_info_log(shader);
    if !info.is_empty() {
        panic!("Shader compile error in `{}`: `{}`", name, info);
    }

    shader
}
