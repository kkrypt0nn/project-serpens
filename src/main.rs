use clap::Parser;

mod events;
mod logger;
mod modules;
mod options;
mod session;

// TODO Rework the options system, same for the overall modules management

fn main() {
    println!("Project Serpens v{}", env!("CARGO_PKG_VERSION"));

    let options = options::Options::parse();

    let mut session = session::Session::new();
    session.register_default_modules();
    session.start();

    session.emit(events::Type::DiscoveredDomain, Some(options))
}
