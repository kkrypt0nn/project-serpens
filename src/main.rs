use clap::Parser;
use flume;

mod config;
mod events;
mod logger;
mod modules;
mod session;

// TODO : Rework the overall modules management
// TODO : Allow the events to contain either arguments or some sort of `Context`

fn main() {
    println!("Project Serpens v{}", env!("CARGO_PKG_VERSION"));

    let config = config::Config::parse();
    let (tx, rx) = flume::bounded::<events::Type>(100);

    let mut session = session::Session::new(config, tx, rx);
    session.register_default_modules();
    session.start();
}
