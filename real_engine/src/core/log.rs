extern crate spdlog;
use std::sync::{Arc, OnceLock};

use spdlog::{critical, error, formatter::{pattern, PatternFormatter}, info, sink::{FileSink, Sink, StdStream, StdStreamSink}, terminal_style::StyleMode, trace, warn};

pub struct Log {
    core_logger: Arc<spdlog::Logger>,
    client_logger: Arc<spdlog::Logger>,
}

impl Log { 
    fn new() -> Self {
        let core_logger: Arc<spdlog::Logger>;
        let client_logger: Arc<spdlog::Logger>;

        let new_formatter: Box<PatternFormatter<_>> = Box::new(PatternFormatter::new(pattern!(
            "{^[{time}] {logger}: {payload}} {eol}"
        )));

        let mut sinks: Vec<Arc<dyn Sink>> = Vec::new();
        sinks.push( Arc::new(
            StdStreamSink::builder()
                .std_stream(StdStream::Stdout)
                .formatter(new_formatter.clone())
                .style_mode(StyleMode::Always)
                .build().unwrap()
        ));
        sinks.push( Arc::new(
            FileSink::builder()
                .truncate(true)
                .formatter(new_formatter)
                .path("logs/RealEngine.log")
                .build().unwrap()
        ));

        core_logger = Arc::new(
            spdlog::Logger::builder()
                .name("RealEngine")
                .sinks(sinks.clone())
                .level_filter(spdlog::LevelFilter::All)
                .build().unwrap()
        );

        client_logger = Arc::new(
            spdlog::Logger::builder()
                .name("APP")
                .sinks(sinks)
                .level_filter(spdlog::LevelFilter::All)
                .build().unwrap()
        );

        return Log { core_logger, client_logger };
    }


    pub fn get_core_logger() -> &'static Arc<spdlog::Logger> {
        static LOG: OnceLock<Log> = OnceLock::new();

        &LOG.get_or_init(|| Log::new() ).core_logger
    }

    pub fn get_client_logger() -> &'static Arc<spdlog::Logger> {
        static LOG: OnceLock<Log> = OnceLock::new();

        &LOG.get_or_init(|| Log::new() ).client_logger
    }

    pub fn print_trace(logger_type: &'static Arc<spdlog::Logger>, fmt_args: std::fmt::Arguments) {
        trace!(logger: logger_type, "{}", fmt_args);
    }
    
    pub fn print_info(logger_type: &'static Arc<spdlog::Logger>, fmt_args: std::fmt::Arguments) {
        info!(logger: logger_type, "{}", fmt_args);
    }

    pub fn print_warn(logger_type: &'static Arc<spdlog::Logger>, fmt_args: std::fmt::Arguments) {
        warn!(logger: logger_type, "{}", fmt_args);
    }

    pub fn print_error(logger_type: &'static Arc<spdlog::Logger>, fmt_args: std::fmt::Arguments) {
        error!(logger: logger_type, "{}", fmt_args);
    }

    pub fn print_critical(logger_type: &'static Arc<spdlog::Logger>, fmt_args: std::fmt::Arguments) {
        critical!(logger: logger_type, "{}", fmt_args);
    }
}

//For use by app
#[macro_export]
macro_rules! re_trace { ($($arg:tt)+) => ($crate::core::log::Log::print_trace($crate::core::log::Log::get_client_logger(), format_args!($($arg)+));) }
#[macro_export]
macro_rules! re_info { ($($arg:tt)+) => ($crate::core::log::Log::print_info($crate::core::log::Log::get_client_logger(), format_args!($($arg)+));) }
#[macro_export]
macro_rules! re_warn { ($($arg:tt)+) => ($crate::core::log::Log::print_warn($crate::core::log::Log::get_client_logger(), format_args!($($arg)+));) }
#[macro_export]
macro_rules! re_error { ($($arg:tt)+) => ($crate::core::log::Log::print_error($crate::core::log::Log::get_client_logger(), format_args!($($arg)+));) }
#[macro_export]
macro_rules! re_critical { ($($arg:tt)+) => ($crate::core::log::Log::print_critical($crate::core::log::Log::get_client_logger(), format_args!($($arg)+));) }


//Don't want to be seen outside of this crate
#[macro_export]
#[doc(hidden)]
macro_rules! re_core_trace { ($($arg:tt)+) => (spdlog::trace!(logger: $crate::core::log::Log::get_core_logger(), $($arg)+);) }
#[macro_export]
#[doc(hidden)]
macro_rules! re_core_info { ($($arg:tt)+) => (spdlog::info!(logger: $crate::core::log::Log::get_core_logger(), $($arg)+);) }
#[macro_export]
#[doc(hidden)]
macro_rules! re_core_warn { ($($arg:tt)+) => (spdlog::warn!(logger: $crate::core::log::Log::get_core_logger(), $($arg)+);) }
#[macro_export]
#[doc(hidden)]
macro_rules! re_core_error { ($($arg:tt)+) => (spdlog::error!(logger: $crate::core::log::Log::get_core_logger(), $($arg)+);) }
#[macro_export]
#[doc(hidden)]
macro_rules! re_core_critical { ($($arg:tt)+) => (spdlog::critical!(logger: $crate::core::log::Log::get_core_logger(), $($arg)+);) }