/// Report a value in a set
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! set {
    ($stat:expr, $val:expr) => {
        $crate::Datadog::set($stat, $val, &[]);
    };
    ($stat:path, $val:expr) => {
        $crate::Datadog::set($stat.as_ref(), $val, &[]);
    };
    ($stat:expr, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::set($stat, $val, &[$(std::format!("{}:{}", $key, $value).as_str()), *]);
    };
    ($stat:path, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::set($stat.as_ref(), $val, &[$(std::format!("{}:{}", $key, $value).as_str()), *]);
    };
}
