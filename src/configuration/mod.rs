pub mod prima;

/// this represents a valid configuration entity
pub trait Configuration {
    fn to_addr(&self) -> &str;
    fn from_addr(&self) -> &str;
    fn namespace(&self) -> &str;
    fn is_reporting_enabled(&self) -> bool;
    fn default_tags(&self) -> Vec<String>;
}

impl Configuration for dogstatsd::Options {
    fn to_addr(&self) -> &str {
        self.to_addr.as_str()
    }

    fn from_addr(&self) -> &str {
        self.from_addr.as_str()
    }

    fn namespace(&self) -> &str {
        self.namespace.as_str()
    }

    fn is_reporting_enabled(&self) -> bool {
        true
    }

    fn default_tags(&self) -> Vec<String> {
        vec![]
    }
}
