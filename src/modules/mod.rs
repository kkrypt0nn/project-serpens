use std::any::Any;

use crate::events;
use crate::session::Session;

pub mod events_log;
pub mod passive_dns;
pub mod ready;

pub trait Module {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn subscribers(&self) -> Vec<events::Type>;
    fn execute(&self, session: &Session, args: &[Box<dyn Any>]);
}
