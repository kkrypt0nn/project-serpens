use rand::Rng;
use reqwest::StatusCode;
use std::fs::File;
use std::io::{BufRead, BufReader};

use reqwest::header::USER_AGENT;

use crate::modules::Module;
use crate::session::Session;
use crate::{events, logger, options};

pub struct ModuleEnumerateFiles {}

impl Default for ModuleEnumerateFiles {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleEnumerateFiles {
    pub fn new() -> Self {
        ModuleEnumerateFiles {}
    }
}

impl Module for ModuleEnumerateFiles {
    fn name(&self) -> String {
        String::from("enumerate:files")
    }

    fn description(&self) -> String {
        String::from("This module will aggressively try to find files based on the given wordlist and extension")
    }

    fn subscribers(&self) -> Vec<events::Type> {
        vec![events::Type::DiscoveredDomain]
    }

    fn execute(&self, _: &Session, opts: &options::Options) {
        let domain = &opts.domain;
        let wordlist_file = File::open(&opts.enumerate_files.enumerate_files_wordlist)
            .expect("Invalid wordlist file path");
        let extension = &opts.enumerate_files.enumerate_files_extension;
        let lines = BufReader::new(wordlist_file).lines();
        for line in lines.map_while(Result::ok) {
            let user_agents_file = include_str!("../../../resources/user_agents.txt");
            let user_agents_lines = user_agents_file.lines();
            let random_user_agent = user_agents_lines.clone().collect::<Vec<_>>()
                [rand::thread_rng().gen_range(0..user_agents_lines.count())];

            let uri = format!(
                "{}/{}{}",
                domain,
                line,
                if extension != "" {
                    ".".to_string() + extension
                } else {
                    "".to_string()
                }
            );
            if let Ok(response) = reqwest::blocking::Client::new()
                .get(format!("https://{}", uri))
                .header(USER_AGENT, random_user_agent)
                .send()
            {
                if response.status() == StatusCode::OK {
                    logger::println(
                        self.name(),
                        format!("Discovered '{}' as an existing file", uri),
                    );
                }
            }
        }
    }
}
