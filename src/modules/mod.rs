use crate::session::Session;
use crate::{events, options};

pub mod enumerate_files;
pub mod enumerate_subdomains;
pub mod passive_dns;
pub mod ready;

pub trait Module {
    fn name(&self) -> String;
    #[allow(dead_code)]
    fn description(&self) -> String;
    fn subscribers(&self) -> Vec<events::Type>;
    fn execute(&self, session: &Session, opts: &options::Options);
}
