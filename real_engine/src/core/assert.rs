#[macro_export]
#[doc(hidden)]
macro_rules! re_core_assert { 
    ($cond:expr) => {
        if cfg!(debug_assertions) {
            $crate::re_core_critical!("TRACE: file: {}, line: {}", file!(), line!());
            panic!();
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        if cfg!(debug_assertions) && !$cond {
            $crate::re_core_critical!("TRACE: file: {}, line: {}", file!(), line!());
            $crate::re_core_critical!($($arg)*);
            panic!();
        }
    };
}

#[macro_export]
macro_rules! re_assert { 
    ($cond:expr) => {
        if cfg!(debug_assertions) {
            $crate::re_core_critical!("TRACE: file: {}, line: {}", file!(), line!());
            panic!();
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        if cfg!(debug_assertions) && !$cond {
            $crate::re_core_critical!("TRACE: file: {}, line: {}", file!(), line!());
            $crate::re_critical!($($arg)*);
            panic!();
        }
    };
}