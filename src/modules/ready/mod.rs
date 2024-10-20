use crate::events;
use crate::logger;
use crate::modules::Module;
use crate::session::Session;

pub struct ModuleReady {}

impl Default for ModuleReady {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleReady {
    pub fn new() -> Self {
        ModuleReady {}
    }
}

impl Module for ModuleReady {
    fn name(&self) -> String {
        String::from("ready")
    }

    fn description(&self) -> String {
        String::from("This module is responsible to know when Project Serpens is ready and will start to do the work")
    }

    fn subscribers(&self) -> Vec<events::Type> {
        vec![events::Type::Ready]
    }

    fn execute(&self, _: &Session) {
        logger::println(
            "ready",
            "Project Serpens is now ready and will start doing its magic!",
        )
    }
}
