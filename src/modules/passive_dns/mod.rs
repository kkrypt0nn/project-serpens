use std::any::Any;
use std::sync::Mutex;

use crate::modules::passive_dns::crt_sh::CrtShItem;
use crate::modules::Module;
use crate::session::Session;
use crate::{events, logger, modules};

mod crt_sh;

pub struct ModulePassiveDNS {
    discovered_domains: Mutex<Vec<String>>,
    processed_domains: Mutex<Vec<String>>,
}

impl Default for ModulePassiveDNS {
    fn default() -> Self {
        Self::new()
    }
}

impl ModulePassiveDNS {
    pub fn new() -> Self {
        ModulePassiveDNS {
            discovered_domains: Mutex::new(Vec::new()),
            processed_domains: Mutex::new(Vec::new()),
        }
    }

    pub fn new_args(domain: String) -> Vec<Box<dyn Any>> {
        vec![Box::new(domain)]
    }

    pub fn process(&self, domain: String) {
        self.processed_domains.lock().unwrap().push(domain)
    }

    pub fn has_processed(&self, domain: String) -> bool {
        self.processed_domains.lock().unwrap().contains(&domain)
    }

    pub fn discover(&self, domain: String) {
        self.discovered_domains.lock().unwrap().push(domain)
    }

    pub fn has_discovered(&self, domain: String) -> bool {
        self.discovered_domains.lock().unwrap().contains(&domain)
    }
}

impl Module for ModulePassiveDNS {
    fn name(&self) -> String {
        String::from("dns:passive")
    }

    fn description(&self) -> String {
        String::from(
            "This module will use crt.sh to discover new domains perform a passive discovery",
        )
    }

    fn subscribers(&self) -> Vec<events::Type> {
        vec![events::Type::DiscoveredDomain]
    }

    fn execute(&self, session: &Session, args: &[Box<dyn Any>]) {
        let domain = args[0]
            .downcast_ref::<&'static str>()
            .expect("Invalid domain");
        if self.has_processed(domain.to_string()) {
            return;
        }

        self.process(domain.to_string());
        let response = reqwest::blocking::Client::new()
            .get(format!("https://crt.sh/?q={}&output=json", domain))
            .send();
        match response {
            Ok(response) => {
                let items: Vec<CrtShItem> = response.json().unwrap_or_default();
                for item in items {
                    let name_value = &item
                        .name_value
                        .split('\n')
                        .map(|x| x.strip_prefix("*.").unwrap_or(x).to_string())
                        .collect::<Vec<String>>()[0];
                    if name_value == &domain.to_string() {
                        continue;
                    }
                    if !self.has_discovered(name_value.to_string()) {
                        let args = modules::events_log::ModuleEventsLog::new_args(
                            "dns:passive",
                            format!("Discovered '{}' as a new subdomain", name_value),
                        );
                        session.emit(events::Type::Log, Option::from(args));
                        self.discover(name_value.to_string());
                    }
                }
            }
            Err(_) => logger::error("dns:passive", "Failed performing a request to crt.sh"),
        }
    }
}
