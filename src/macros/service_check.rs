/// Report the status of a service
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! service_check {
    // call with literal and status
    ($stat:expr, $service_status:path) => {
        $crate::Datadog::service_check($stat, $service_status, vec![], None);
    };
    ($stat:expr, $service_status:path, $options: expr) => {
        $crate::Datadog::service_check($stat, $service_status, vec![], Some($options));
    };
    // call with path, status and options
    ($stat:path, $service_status:path) => {
        $crate::Datadog::service_check($stat.as_ref(), $service_status, vec![], None);
    };
    ($stat:path, $service_status:path, $options: expr) => {
        $crate::Datadog::service_check($stat.as_ref(), $service_status, vec![], Some($options));
    };
    // call with literal, status, options and tags
    ($stat:expr, $service_status:path; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::service_check($stat, $service_status, std::vec![$(std::format!("{}:{}", $key, $value)), *], None);
    };
    ($stat:expr, $service_status:path, $options: expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::service_check($stat, $service_status, std::vec![$(std::format!("{}:{}", $key, $value)), *], Some($options));
    };
    // call with path, status, options and tags
    ($stat:path, $service_status:path; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::service_check($stat.as_ref(), $service_status, std::vec![$(std::format!("{}:{}", $key, $value)), *], None);
    };
    ($stat:path, $service_status:path, $options: expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::service_check($stat.as_ref(), $service_status, std::vec![$(std::format!("{}:{}", $key, $value)), *], Some($options));
    };
}
