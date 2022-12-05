/// Run an experiment comparing the execution time of two blocks of code
/// Example:
/// ```
/// # use rand::prelude::*;
/// # use prima_datadog::compare;
/// # use std::thread::sleep;
/// # use std::time::Duration;
/// # let mut rng = thread_rng();
/// // This could also be a network request to an experiment management service, or anything you want
/// let path = rng.gen_range(0..2);
/// compare!("test", path, || {
///     sleep(Duration::from_millis(rng.gen_range(5..10)));
/// }, || {
///     sleep(Duration::from_millis(rng.gen_range(5..12)));
/// }; "some" => "tag");
/// ```
/// The above code will run the first block passed, and after execution,
/// will emit a timing metric to datadog. The metric will be named with
/// the value of EXPERIMENTS_METRIC_NAME, and will be tagged with the name
/// of the experiment ("experiment_name:test"), the path taken ("path_taken:0"),
/// and any additional tags provided ("some:tag").
///
/// The blocks can be arbitrary code, and the macro is async-safe. In an async context,
/// the timing will continue across await points, which means that if, for example, you
/// are awaiting a network request, the timing will include the time spent waiting for
/// the network request to complete.
///
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! compare {
    ($name:expr, $path_taken:expr, || $block_1:expr, || $block_2:expr) => {
    {
        use $crate::timing_guard::EXPERIMENTS_METRIC_NAME;
        let prima_datadog_experiment_tags = &[::std::format!("experiment_name:{}", $name), ::std::format!("path_taken:{}", $path_taken)];
        let _prima_datadog_timing_guard = $crate::Datadog::enter_timing(EXPERIMENTS_METRIC_NAME, prima_datadog_experiment_tags);
        if $path_taken == 0 {
            $block_1
        } else {
            $block_2
        }
    }
    };
    ($name:expr, $path_taken:expr, move || $block_1:expr, move || $block_2:expr) => {
    {
        use $crate::timing_guard::EXPERIMENTS_METRIC_NAME;
        let prima_datadog_experiment_tags = &[::std::format!("experiment_name:{}", $name), ::std::format!("path_taken:{}", $path_taken)];
        let _prima_datadog_timing_guard = $crate::Datadog::enter_timing(EXPERIMENTS_METRIC_NAME, prima_datadog_experiment_tags);
        if $path_taken == 0 {
            $block_1
        } else {
            $block_2
        }
    }
    };
    ($name:expr, $path_taken:expr, || $block_1:expr, || $block_2:expr; $( $key:expr => $value:expr ), *) => {
    {
        use $crate::timing_guard::EXPERIMENTS_METRIC_NAME;
        let prima_datadog_experiment_tags = &[::std::format!("experiment_name:{}", $name), ::std::format!("path_taken:{}", $path_taken), $(::std::format!("{}:{}", $key, $value)), *];
        let _prima_datadog_timing_guard = $crate::Datadog::enter_timing(EXPERIMENTS_METRIC_NAME, prima_datadog_experiment_tags);
        if $path_taken == 0 {
            $block_1
        } else {
            $block_2
        }
    }
    };
    ($name:expr, $path_taken:expr, move || $block_1:expr, move || $block_2:expr; $( $key:expr => $value:expr ), *) => {
    {
        use $crate::timing_guard::EXPERIMENTS_METRIC_NAME;
        let prima_datadog_experiment_tags = &[::std::format!("experiment_name:{}", $name), ::std::format!("path_taken:{}", $path_taken), $(::std::format!("{}:{}", $key, $value)), *];
        let _prima_datadog_timing_guard = $crate::Datadog::enter_timing(EXPERIMENTS_METRIC_NAME, prima_datadog_experiment_tags);
        if $path_taken == 0 {
            $block_1
        } else {
            $block_2
        }
    }
    };
}
