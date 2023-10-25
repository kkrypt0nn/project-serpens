extern crate kernel;

use std::any::Any;

fn main() {
    let mut session = kernel::session::Session::new();
    session.register_default_modules();
    session.start();

    let domain_args: Vec<Box<dyn Any>> = vec![Box::new("krypton.ninja")];
    session.emit(
        kernel::events::Type::DiscoveredDomain,
        Option::from(domain_args),
    )
}
