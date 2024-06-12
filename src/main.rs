use clap::Parser;
use std::any::Any;

mod options;

// TODO Rework the options system

fn main() {
    println!("Project Serpens v{}", env!("CARGO_PKG_VERSION"));

    let options = options::Options::parse();

    let mut session = kernel::session::Session::new();
    session.register_default_modules();
    session.start();

    let domain_args: Vec<Box<dyn Any>> = vec![
        Box::new(options.domain),
        Box::new(options.passive_dns.passive_dns_ignore_expired),
        Box::new(options.passive_dns.passive_dns_recent_only),
    ];
    session.emit(
        kernel::events::Type::DiscoveredDomain,
        Option::from(domain_args),
    )
}
