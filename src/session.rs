use std::sync::Mutex;

use flume::{Receiver, Sender};

use crate::modules::Module;
use crate::{events, logger, modules, options};

pub struct Session {
    dev_mode: bool,
    options: options::Options,
    sender: Sender<events::Type>,
    receiver: Receiver<events::Type>,

    modules: Vec<Box<dyn Module>>,
    discovered_subdomains: Mutex<Vec<String>>,
}

impl Session {
    pub fn new(
        options: options::Options,
        sender: Sender<events::Type>,
        receiver: Receiver<events::Type>,
    ) -> Self {
        Session {
            dev_mode: false,
            options,
            sender,
            receiver,
            modules: Vec::new(),
            discovered_subdomains: Mutex::new(Vec::new()),
        }
    }

    pub fn discover_subdomain(&self, subdomain: String) {
        self.discovered_subdomains.lock().unwrap().push(subdomain)
    }

    pub fn has_discovered_subdomain(&self, subdomain: String) -> bool {
        self.discovered_subdomains
            .lock()
            .unwrap()
            .contains(&subdomain)
    }

    pub fn register_module<T: Module + 'static>(&mut self, module: T) {
        if self.dev_mode {
            logger::debug("", format!("Registered module {}", module.name()))
        }
        self.modules.push(Box::new(module));
    }

    // #[allow(dead_code)]
    // pub fn register_modules<T: Module + 'static>(&mut self, modules: Vec<T>) {
    //     for module in modules {
    //         if self.dev_mode {
    //             logger::debug("", format!("Registered module {}", module.name()))
    //         }
    //         self.modules.push(Box::new(module));
    //     }
    // }

    pub fn register_default_modules(&mut self) {
        self.register_module(modules::ready::ModuleReady::new());
        // self.register_module(modules::enumerate_files::ModuleEnumerateFiles::new());
        // self.register_module(modules::enumerate_subdomains::ModuleEnumerateSubdomains::new());
        // self.register_module(modules::passive_dns::ModulePassiveDNS::new());
    }

    pub fn emit(&self, event: events::Type, args: Option<options::Options>) {
        let (session, opts) = (self, &args.unwrap_or_default());
        self.sender.send(event);
    }

    pub fn start(&mut self) {
        self.emit(events::Type::Ready, None);

        while let Ok(event) = self.receiver.recv() {
            for module in &self.modules {
                if module.subscribers().contains(&event) {
                    module.execute(&self, &self.options);
                }
            }
        }
    }
}
