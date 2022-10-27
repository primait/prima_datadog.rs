/// Make an arbitrary change to a StatsD counter
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! count {
    ($stat:expr, $count:expr) => {
        $crate::Datadog::count($stat, $count, &[]);
    };
    ($stat:path, $count:expr) => {
        $crate::Datadog::count($stat.as_ref(), $count, &[]);
    };
    ($stat:expr, $count:expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::count($stat, $count, &[$(::core::concat!($key, ":", $value)), *]);
    };
    ($stat:path, $count:expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::count($stat.as_ref(), $count, &[$(::core::concat!($key, ":", $value)), *]);
    };
    ($stat:expr, $count:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::count($stat, $count, &[$(::core::format!("{}:{}", $key, $value).as_str()), *]);
    };
    ($stat:path, $count:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::count($stat.as_ref(), $count, &[$(::core::format!("{}:{}", $key, $value).as_str()), *]);
    };
}
