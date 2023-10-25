use std::fmt;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Log,
    Ready,
    DiscoveredDomain,
}

impl fmt::Display for Type {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Log => {
                write!(formatter, "log")
            }
            Type::Ready => {
                write!(formatter, "ready")
            }
            Type::DiscoveredDomain => {
                write!(formatter, "discovered:domain")
            }
        }
    }
}
