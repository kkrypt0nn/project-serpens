use std::any::Any;

use crate::events;
use crate::logger;
use crate::modules::Module;
use crate::session::Session;

pub struct ModuleEventsLog {}

impl Default for ModuleEventsLog {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleEventsLog {
    pub fn new() -> Self {
        ModuleEventsLog {}
    }

    pub fn new_args(name: impl Into<String>, message: impl Into<String>) -> Vec<Box<dyn Any>> {
        vec![Box::new(name.into()), Box::new(message.into())]
    }
}

impl Module for ModuleEventsLog {
    fn name(&self) -> String {
        String::from("events.log")
    }

    fn description(&self) -> String {
        String::from("This module handles the logs to perform")
    }

    fn subscribers(&self) -> Vec<events::Type> {
        vec![events::Type::Log]
    }

    fn execute(&self, _: &Session, args: &[Box<dyn Any>]) {
        let name = args[0]
            .downcast_ref::<String>()
            .expect("Invalid event name to be logged");
        let message = args[1]
            .downcast_ref::<String>()
            .expect("Invalid event message to be logged");
        logger::println(name, message);
    }
}
