use chrono::{Duration, Utc};
use rand::Rng;
use std::any::Any;
use std::sync::Mutex;

use reqwest::header::USER_AGENT;

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
            "This module will perform a passive discovery of new subdomains by using crt.sh",
        )
    }

    fn subscribers(&self) -> Vec<events::Type> {
        vec![events::Type::DiscoveredDomain]
    }

    fn execute(&self, session: &Session, args: &[Box<dyn Any>]) {
        let domain = args[0].downcast_ref::<String>().expect("Invalid domain");
        let ignore_expired = args[1].downcast_ref::<bool>().unwrap_or(&false);
        let recent_only = args[2].downcast_ref::<bool>().unwrap_or(&false);
        if self.has_processed(domain.to_string()) {
            return;
        }
        self.process(domain.to_string());

        let file = include_str!("../../../resources/user_agents.txt");
        let lines = file.lines();
        let random_user_agent =
            lines.clone().collect::<Vec<_>>()[rand::thread_rng().gen_range(0..lines.count())];

        let response = reqwest::blocking::Client::new()
            .get(format!("https://crt.sh/?q={}&output=json", domain))
            .header(USER_AGENT, random_user_agent)
            .send();
        match response {
            Ok(response) => {
                let items: Vec<CrtShItem> = response.json().unwrap_or_default();
                for item in items {
                    let name_values = &item
                        .name_value
                        .split('\n')
                        .map(|x| x.strip_prefix("*.").unwrap_or(x).to_string())
                        .collect::<Vec<String>>();
                    for name_value in name_values {
                        if name_value == &domain.to_string() {
                            continue;
                        }
                        if !self.has_discovered(name_value.to_string()) {
                            let now = Utc::now();

                            // Check if the certificate has expired
                            let has_expired = item.not_after < now;
                            if *ignore_expired && has_expired {
                                continue;
                            }

                            // Check if the certificate has been created within the last 24 hours
                            let is_recent = item.not_before <= now
                                && item.not_before >= now - Duration::try_hours(24).unwrap();
                            if !is_recent && *recent_only {
                                continue;
                            }

                            let args = modules::events_log::ModuleEventsLog::new_args(
                                "dns:passive",
                                format!(
                                    "Discovered '{}' as a new subdomain{}{}",
                                    name_value,
                                    if has_expired {
                                        " $[fg:red]$[effect:bold](Certificate expired, likely inactive)"
                                    } else {
                                        ""
                                    },
                                    if is_recent {
                                        " $[fg:blue]$[effect:bold](Active since less than 24 hours)"
                                    } else {
                                        ""
                                    }
                                ),
                            );
                            session.emit(events::Type::Log, Option::from(args));
                            self.discover(name_value.to_string());
                        }
                    }
                }
            }
            Err(_) => logger::error("dns:passive", "Failed performing a request to crt.sh"),
        }
    }
}
