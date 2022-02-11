pub enum TestEvent {
    Test1,
}

impl AsRef<str> for TestEvent {
    fn as_ref(&self) -> &str {
        match self {
            TestEvent::Test1 => "test1_event",
        }
    }
}

impl std::fmt::Display for TestEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[cfg(feature = "noop")]
pub enum TestEvent2 {
    Test2,
}

#[cfg(feature = "noop")]
impl AsRef<str> for TestEvent2 {
    fn as_ref(&self) -> &str {
        match self {
            TestEvent2::Test2 => "test2_event",
        }
    }
}

#[cfg(feature = "noop")]
impl std::fmt::Display for TestEvent2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
