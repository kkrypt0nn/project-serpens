use std::any::Any;

use crate::events;
use crate::modules;
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

    fn execute(&self, session: &Session, _: &[Box<dyn Any>]) {
        let args = modules::events_log::ModuleEventsLog::new_args(
            "ready",
            "Project Serpens is now ready and will start doing its magic!",
        );
        session.emit(events::Type::Log, Option::from(args))
    }
}
