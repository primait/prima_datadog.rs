/// Report the status of a service
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! service_check {
    // call with literal and status
    ($stat:expr, $service_status:path) => {
        $crate::Datadog::service_check($stat, $service_status, $crate::EMPTY_TAGS, None);
    };
    ($stat:expr, $service_status:path, $options: expr) => {
        $crate::Datadog::service_check($stat, $service_status, $crate::EMPTY_TAGS, Some($options));
    };
    // call with path, status and options
    ($stat:path, $service_status:path) => {
        $crate::Datadog::service_check($stat.as_ref(), $service_status, $crate::EMPTY_TAGS, None);
    };
    ($stat:path, $service_status:path, $options: expr) => {
        $crate::Datadog::service_check($stat.as_ref(), $service_status, $crate::EMPTY_TAGS, Some($options));
    };
    // call with literal, status, options and tags
    ($stat:expr, $service_status:path; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::service_check($stat, $service_status, &[$(::core::concat!($key, ":", $value)), *], None);
    };
    ($stat:expr, $service_status:path, $options: expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::service_check($stat, $service_status, &[$(::core::concat!($key, ":", $value)), *], Some($options));
    };
    // call with path, status, options and tags
    ($stat:path, $service_status:path; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::service_check($stat.as_ref(), $service_status, &[$(::core::concat!($key, ":", $value)), *], None);
    };
    ($stat:path, $service_status:path, $options: expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::service_check($stat.as_ref(), $service_status, &[$(::core::concat!($key, ":", $value)), *], Some($options));
    };
    // call with literal, status, options and tags
    ($stat:expr, $service_status:path; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::service_check($stat, $service_status, &[$(::core::format!("{}:{}", $key, $value).as_str()), *], None);
    };
    ($stat:expr, $service_status:path, $options: expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::service_check($stat, $service_status, &[$(::core::format!("{}:{}", $key, $value).as_str()), *], Some($options));
    };
    // call with path, status, options and tags
    ($stat:path, $service_status:path; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::service_check($stat.as_ref(), $service_status, &[$(::core::format!("{}:{}", $key, $value).as_str()), *], None);
    };
    ($stat:path, $service_status:path, $options: expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::service_check($stat.as_ref(), $service_status, &[$(::core::format!("{}:{}", $key, $value).as_str()), *], Some($options));
    };
}
