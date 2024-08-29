use chrono::{Duration, Utc};
use rand::Rng;
use std::sync::Mutex;

use reqwest::header::USER_AGENT;

use crate::modules::passive_dns::crt_sh::CrtShItem;
use crate::modules::Module;
use crate::session::Session;
use crate::{events, logger, options};

mod crt_sh;

pub struct ModulePassiveDNS {
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
            processed_domains: Mutex::new(Vec::new()),
        }
    }

    pub fn process(&self, domain: String) {
        self.processed_domains.lock().unwrap().push(domain)
    }

    pub fn has_processed(&self, domain: String) -> bool {
        self.processed_domains.lock().unwrap().contains(&domain)
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

    fn execute(&self, session: &Session, opts: &options::Options) {
        let domain = &opts.domain;
        let ignore_expired = opts.passive_dns.passive_dns_ignore_expired;
        let recent_only = opts.passive_dns.passive_dns_recent_only;
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
                        if !session.has_discovered_subdomain(name_value.to_string()) {
                            let now = Utc::now();

                            // Check if the certificate has expired
                            let has_expired = item.not_after < now;
                            if ignore_expired && has_expired {
                                continue;
                            }

                            // Check if the certificate has been created within the last 24 hours
                            let is_recent = item.not_before <= now
                                && item.not_before >= now - Duration::try_hours(24).unwrap();
                            if !is_recent && recent_only {
                                continue;
                            }

                            logger::println(
                                self.name(),
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
                            session.discover_subdomain(name_value.to_string());
                        }
                    }
                }
            }
            Err(_) => logger::error(self.name(), "Failed performing a request to crt.sh"),
        }
    }
}
