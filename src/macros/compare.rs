/// Run an experiment comparing the execution time of two blocks of code
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! compare {
    ($name:expr, $path_taken:expr, || $block_1:expr, || $block_2:expr) => {
        if $path_taken == 0 {
            let _timing_guard = $crate::Datadog::enter_compare($name, $path_taken, $crate::EMPTY_TAGS);
            $block_1
        } else {
            let _timing_guard = $crate::Datadog::enter_compare($name, $path_taken, $crate::EMPTY_TAGS);
            $block_2
        }
    };
    ($name:expr, $path_taken:expr, move || $block_1:expr, move || $block_2:expr) => {
        if $path_taken == 0 {
            let _timing_guard = $crate::Datadog::enter_compare($name, $path_taken, $crate::EMPTY_TAGS);
            $block_1
        } else {
            let _timing_guard = $crate::Datadog::enter_compare($name, $path_taken, $crate::EMPTY_TAGS);
            $block_2
        }
    };
    ($name:path, $path_taken:expr, || $block_1:expr, || $block_2:expr) => {
        if $path_taken == 0 {
            let _timing_guard = $crate::Datadog::enter_compare($name.as_ref() $path_taken, $crate::EMPTY_TAGS);
            $block_1
        } else {
            let _timing_guard = $crate::Datadog::enter_compare($name.as_ref(), $path_taken, $crate::EMPTY_TAGS);
            $block_2
        }
    };
    ($name:path, $path_taken:expr, move || $block_1:expr, move || $block_2:expr) => {
        if $path_taken == 0 {
            let _timing_guard = $crate::Datadog::enter_compare($name.as_ref(), $path_taken, $crate::EMPTY_TAGS);
            $block_1
        } else {
            let _timing_guard = $crate::Datadog::enter_compare($name.as_ref(), $path_taken, $crate::EMPTY_TAGS);
            $block_2
        }
    };
    ($name:expr, $path_taken:expr, || $block_1:expr, || $block_2:expr; $( $key:expr => $value:expr ), *) => {
        if $path_taken == 0 {
            let _timing_guard = $crate::Datadog::enter_compare($name, $path_taken, &[$(::std::format!("{}:{}", $key, $value).as_str()), *]);
            $block_1
        } else {
            let _timing_guard = $crate::Datadog::enter_compare($name, $path_taken, $crate::EMPTY_TAGS);
            $block_2
        }
    };
    ($name:expr, $path_taken:expr, move || $block_1:expr, move || $block_2:expr; $( $key:expr => $value:expr ), *) => {
        if $path_taken == 0 {
            let _timing_guard = $crate::Datadog::enter_compare($name, $path_taken, &[$(::std::format!("{}:{}", $key, $value).as_str()), *]);
            $block_1
        } else {
            let _timing_guard = $crate::Datadog::enter_compare($name, $path_taken, $crate::EMPTY_TAGS);
            $block_2
        }
    };
    ($name:path, $path_taken:expr, || $block_1:expr, || $block_2:expr; $( $key:expr => $value:expr ), *) => {
        if $path_taken == 0 {
            let _timing_guard = $crate::Datadog::enter_compare($name.as_ref(), $path_taken, &[$(::std::format!("{}:{}", $key, $value).as_str()), *]);
            $block_1
        } else {
            let _timing_guard = $crate::Datadog::enter_compare($name.as_ref(), $path_taken, $crate::EMPTY_TAGS);
            $block_2
        }
    };
    ($name:path, $path_taken:expr, move || $block_1:expr, move || $block_2:expr; $( $key:expr => $value:expr ), *) => {
        if $path_taken == 0 {
            let _timing_guard = $crate::Datadog::enter_compare($name.as_ref(), $path_taken, &[$(::std::format!("{}:{}", $key, $value).as_str()), *]);
            $block_1
        } else {
            let _timing_guard = $crate::Datadog::enter_compare($name.as_ref(), $path_taken, $crate::EMPTY_TAGS);
            $block_2
        }
    };
}
