use super::element::ElementKind;

pub struct ToolHub {
    logic: Box<dyn HubLogic>,
}

impl ToolHub {
    pub fn init(_ctx: Context) -> Self {
        Self {
            logic: Box::new(Dummy),
        }
    }

    pub fn process(&mut self, _ctx: Context) {}

    pub fn change_logic(&mut self, _kind: ElementKind) {}
}

pub struct Context<'u> {
    pub up: &'u super::Context<'u>,
}

trait HubLogic {
    fn tool(&self, _ctx: &Context) -> Option<Box<dyn Tool>>;
}

trait Tool {
    fn process(&mut self, _ctx: &Context) {}
}

struct Dummy;

impl HubLogic for Dummy {
    fn tool(&self, _ctx: &Context) -> Option<Box<dyn Tool>> {
        None
    }
}
