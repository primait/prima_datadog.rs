/// Send a custom event as a title and a body
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! event {
    ($stat:path, $text:expr) => {
        $crate::Datadog::event($stat.as_ref(), $text, vec![]);
    };
    ($stat:expr, $text:expr) => {
        $crate::Datadog::event($stat, $text, vec![]);
    };
    ($stat:expr, $text:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::event($stat, $text, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
    ($stat:path, $text:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::event($stat.as_ref(), $text, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
}
