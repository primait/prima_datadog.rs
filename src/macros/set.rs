/// Report a value in a set
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! set {
    ($stat:expr, $val:expr) => {
        $crate::Datadog::set($stat, $val, $crate::EMPTY_TAGS);
    };
    ($stat:path, $val:expr) => {
        $crate::Datadog::set($stat.as_ref(), $val, $crate::EMPTY_TAGS);
    };
    ($stat:expr, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::set($stat, $val, &[$(::core::concat!($key, ":", $value)), *]);
    };
    ($stat:path, $val:expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::set($stat.as_ref(), $val, &[$(::core::concat!($key, ":", $value)), *]);
    };
    ($stat:expr, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::set($stat, $val, &[$(::core::format!("{}:{}", $key, $value).as_str()), *]);
    };
    ($stat:path, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::set($stat.as_ref(), $val, &[$(::core::format!("{}:{}", $key, $value).as_str()), *]);
    };
}
