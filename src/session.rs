use std::any::Any;

use crate::modules::Module;
use crate::{events, logger, modules};

pub struct Session {
    pub dev_mode: bool,
    modules: Vec<Box<dyn Module>>,
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

impl Session {
    pub fn new() -> Self {
        Session {
            dev_mode: false,
            modules: Vec::new(),
        }
    }

    pub fn register_module<T: Module + 'static>(&mut self, module: T) {
        if self.dev_mode {
            logger::debug("", format!("Registered module {}", module.name()))
        }
        self.modules.push(Box::new(module));
    }

    pub fn register_modules<T: Module + 'static>(&mut self, modules: Vec<T>) {
        for module in modules {
            if self.dev_mode {
                logger::debug("", format!("Registered module {}", module.name()))
            }
            self.modules.push(Box::new(module));
        }
    }

    pub fn register_default_modules(&mut self) {
        self.register_module(modules::ready::ModuleReady::new());
        self.register_module(modules::events_log::ModuleEventsLog::new());
        self.register_module(modules::passive_dns::ModulePassiveDNS::new());
    }

    pub fn emit(&self, name: events::Type, args: Option<Vec<Box<dyn Any>>>) {
        let (session, args) = (self, &args.unwrap_or_default());
        for module in &self.modules {
            if module.subscribers().contains(&name) {
                module.execute(session, args);
            }
        }
    }

    pub fn start(&mut self) {
        self.emit(events::Type::Ready, None);
    }
}
