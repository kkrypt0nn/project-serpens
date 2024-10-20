use clap::Parser;
use flume;

mod events;
mod logger;
mod modules;
mod options;
mod session;

// TODO : Rework the options system, same for the overall modules management
// TODO : Allow the events to contain either arguments or some sort of `Context`

fn main() {
    println!("Project Serpens v{}", env!("CARGO_PKG_VERSION"));

    let options = options::Options::parse();
    let (tx, rx) = flume::bounded::<events::Type>(100);
    tx.send(events::Type::DiscoveredDomain).unwrap();

    let mut session = session::Session::new(options, tx, rx);
    session.register_default_modules();
    session.start();

    // while let Ok(event) = rx.recv() {
    //     println!("{}", event)
    // }

    // session.emit(events::Type::DiscoveredDomain, Some(options));
}
