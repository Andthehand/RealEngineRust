extern crate spdlog;
use std::sync::{Arc, OnceLock};

use spdlog::{formatter::{pattern, PatternFormatter}, info, sink::{FileSink, Sink, StdStream, StdStreamSink}, terminal_style::StyleMode};

pub struct Log {
    core_logger: Arc<spdlog::Logger>,
    client_logger: Arc<spdlog::Logger>,
}

impl Log { 
    pub fn get() -> &'static Log {
        static LOG: OnceLock<Log> = OnceLock::new();

        LOG.get_or_init(|| {
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
                    .build().unwrap()
            );
    
            client_logger = Arc::new(
                spdlog::Logger::builder()
                    .name("APP")
                    .sinks(sinks)
                    .build().unwrap()
            );

            return Log { core_logger, client_logger };
        })
    }

    pub fn print() {
        info!(logger: Log::get().client_logger, "connect to the update server");
    }
}