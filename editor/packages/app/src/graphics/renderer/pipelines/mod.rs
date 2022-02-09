use gpu::{Gpu, Pipeline, Surface};

mod line;
mod prop;
mod solid;

pub struct Pipelines {
    pub line: Pipeline,
    pub solid: Pipeline,
    pub prop: Pipeline,
}

impl Pipelines {
    pub fn new(gpu: &Gpu, surface: &Surface) -> Self {
        Self {
            line: line::pipeline(gpu, surface),
            solid: solid::pipeline(gpu, surface),
            prop: prop::pipeline(gpu, surface),
        }
    }
}
