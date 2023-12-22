use clap::Parser;
use std::any::Any;

mod options;

fn main() {
    println!("Project Absence v{}", env!("CARGO_PKG_VERSION"));

    let options = options::Options::parse();

    let mut session = kernel::session::Session::new();
    session.register_default_modules();
    session.start();

    let domain_args: Vec<Box<dyn Any>> = vec![Box::new(options.domain)];
    session.emit(
        kernel::events::Type::DiscoveredDomain,
        Option::from(domain_args),
    )
}
