/// Send a custom event as a title and a body
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! event {
    ($stat:path, $text:expr) => {
        $crate::Datadog::event($stat.as_ref(), $text, $crate::EMPTY_TAGS);
    };
    ($stat:path, $text:expr, $options:expr) => {
        $crate::Datadog::event_with_options($stat.as_ref(), $text, $crate::EMPTY_TAGS, Some($options));
    };
    ($stat:expr, $text:expr) => {
        $crate::Datadog::event($stat, $text, $crate::EMPTY_TAGS);
    };
    ($stat:expr, $text:expr, $options:expr) => {
        $crate::Datadog::event_with_options($stat, $text, $crate::EMPTY_TAGS, Some($options));
    };
    ($stat:expr, $text:expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::event($stat, $text, &[$(::core::concat!($key, ":", $value)), *]);
    };
    ($stat:expr, $text:expr, $options:expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::event_with_options($stat, $text, &[$(::core::concat!($key, ":", $value)), *], Some($options));
    };
    ($stat:path, $text:expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::event($stat.as_ref(), $text, &[$(::core::concat!($key, ":", $value)), *]);
    };
    ($stat:path, $text:expr, $options:expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::event_with_options($stat.as_ref(), $text, &[$(::core::concat!($key, ":", $value)), *], Some($options));
    };
    ($stat:expr, $text:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::event($stat, $text, &[$(::std::format!("{}:{}", $key, $value).as_str()), *]);
    };
    ($stat:expr, $text:expr, $options:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::event_with_options($stat, $text, &[$(::std::format!("{}:{}", $key, $value).as_str()), *], Some($options));
    };
    ($stat:path, $text:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::event($stat.as_ref(), $text, &[$(::std::format!("{}:{}", $key, $value).as_str()), *]);
    };
    ($stat:path, $text:expr, $options:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::event_with_options($stat.as_ref(), $text, &[$(::std::format!("{}:{}", $key, $value).as_str()), *], Some($options));
    };
}
