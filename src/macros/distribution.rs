/// Report a value in a distribution
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! distribution {
    ($stat:expr, $val:expr) => {
        $crate::Datadog::distribution($stat, $val, &[]);
    };
    ($stat:path, $val:expr) => {
        $crate::Datadog::distribution($stat.as_ref(), $val, &[]);
    };
    ($stat:expr, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::distribution($stat, $val, &[$(::core::concat!($key, ":", $value)), *]);
    };
    ($stat:path, $val:expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::distribution($stat.as_ref(), $val, &[$(::core::concat!($key, ":", $value)), *]);
    };
    ($stat:expr, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::distribution($stat, $val, &[$(::core::format!("{}:{}", $key, $value).as_str()), *]);
    };
    ($stat:path, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::distribution($stat.as_ref(), $val, &[$(::core::format!("{}:{}", $key, $value).as_str()), *]);
    };
}
