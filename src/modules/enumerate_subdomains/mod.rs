use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};

use reqwest::header::USER_AGENT;

use crate::modules::Module;
use crate::session::Session;
use crate::{events, logger, options};

pub struct ModuleEnumerateSubdomains {}

impl Default for ModuleEnumerateSubdomains {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleEnumerateSubdomains {
    pub fn new() -> Self {
        ModuleEnumerateSubdomains {}
    }
}

impl Module for ModuleEnumerateSubdomains {
    fn name(&self) -> String {
        String::from("enumerator:subdomains")
    }

    fn description(&self) -> String {
        String::from(
            "This module will aggressively try to find subdomains based on the given wordlist",
        )
    }

    fn subscribers(&self) -> Vec<events::Type> {
        vec![events::Type::DiscoveredDomain]
    }

    fn execute(&self, session: &Session, opts: &options::Options) {
        let domain = &opts.domain;
        let wordlist_file = File::open(&opts.enumerate_subdomains.enumerate_subdomains_wordlist)
            .expect("Invalid wordlist file path");
        let lines = BufReader::new(wordlist_file).lines();
        for line in lines.map_while(Result::ok) {
            let user_agents_file = include_str!("../../../resources/user_agents.txt");
            let user_agents_lines = user_agents_file.lines();
            let random_user_agent = user_agents_lines.clone().collect::<Vec<_>>()
                [rand::thread_rng().gen_range(0..user_agents_lines.count())];

            let uri = format!("{}.{}", line, domain);
            if reqwest::blocking::Client::new()
                .get(format!("https://{}", uri))
                .header(USER_AGENT, random_user_agent)
                .send()
                .is_ok()
                && !session.has_discovered_subdomain(uri.clone())
            {
                logger::println(
                    self.name(),
                    format!("Discovered '{}' as a new subdomain", uri),
                );
                session.discover_subdomain(uri);
            }
        }
    }
}