use std::fmt;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Ready,
    FinishedTask,
    /// String: the discovered domain
    DiscoveredDomain(String),
    /// String: the domain
    /// usize: the port that is open
    OpenPort(String, usize),
}

impl fmt::Display for Type {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Ready => {
                write!(formatter, "ready")
            }
            Type::FinishedTask => {
                write!(formatter, "finished:task")
            }
            Type::DiscoveredDomain(_) => {
                write!(formatter, "discovered:domain")
            }
            Type::OpenPort(_, _) => {
                write!(formatter, "open:port")
            }
        }
    }
}
