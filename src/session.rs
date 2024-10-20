use std::sync::Mutex;

use flume::{Receiver, Sender};

use crate::modules::Module;
use crate::{config, events, logger, modules};

pub struct Session {
    config: config::Config,
    sender: Sender<events::Type>,
    receiver: Receiver<events::Type>,

    modules: Vec<Box<dyn Module>>,
    discovered_subdomains: Mutex<Vec<String>>,

    dev_mode: bool,
}

impl Session {
    pub fn new(
        config: config::Config,
        sender: Sender<events::Type>,
        receiver: Receiver<events::Type>,
    ) -> Self {
        Session {
            config,
            sender,
            receiver,

            modules: Vec::new(),
            discovered_subdomains: Mutex::new(Vec::new()),

            dev_mode: false,
        }
    }

    pub fn get_config(&self) -> &config::Config {
        &self.config
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

    pub fn register_default_modules(&mut self) {
        self.register_module(modules::ready::ModuleReady::new());
        // self.register_module(modules::enumerate_files::ModuleEnumerateFiles::new());
        // self.register_module(modules::enumerate_subdomains::ModuleEnumerateSubdomains::new());
        self.register_module(modules::passive_dns::ModulePassiveDNS::new());
    }

    pub fn emit(&self, event: events::Type) {
        self.sender.send(event);
    }

    pub fn start(&mut self) {
        self.emit(events::Type::Ready);
        self.emit(events::Type::DiscoveredDomain(self.config.domain.clone()));

        while let Ok(event) = self.receiver.recv() {
            for module in &self.modules {
                if module
                    .subscribers()
                    .iter()
                    .any(|sub_event| match (sub_event, &event) {
                        (events::Type::Ready, events::Type::Ready) => true,
                        (events::Type::DiscoveredDomain(_), events::Type::DiscoveredDomain(_)) => {
                            true
                        }
                        _ => false,
                    })
                {
                    module.execute(&self);
                }
            }
        }
    }
}
