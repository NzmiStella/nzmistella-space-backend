// ********************* import ********************* //
use anyhow::{Context, Result};
use chrono::Local;
use serde::Deserialize;
use tracing_appender::{
    non_blocking,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{
    fmt::{
        format::{format, Writer /*, JsonFields*/},
        layer,
        time::FormatTime,
    },
    prelude::*,
    registry, EnvFilter,
};

// ********************* content ********************* //
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct LogConfig {
    // filter
    pub log_level: String,
    // writer
    pub log_dir: String,
    pub log_file_prefix: String,
    pub log_file_suffix: String,
    pub max_log_files: usize,
    // formatter
    pub with_thread_info: bool,
    pub with_ansi: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            log_level: "info".into(),
            log_dir: "./logs".into(),
            log_file_prefix: "space-backend".into(),
            log_file_suffix: "log".into(),
            max_log_files: 14,
            with_thread_info: false,
            with_ansi: false,
        }
    }
}

pub fn init_logging(cfg: &LogConfig) -> Result<()> {
    struct LogTimer;
    impl FormatTime for LogTimer {
        fn format_time(&self, w: &mut Writer) -> std::fmt::Result {
            write!(w, "{}", Local::now().format("[%Y-%m-%d %H:%M:%S]"))
        }
    }
    let formatter = format()
        .with_timer(&LogTimer)
        .with_thread_names(cfg.with_thread_info)
        .with_thread_ids(cfg.with_thread_info)
        .with_ansi(cfg.with_ansi)
        // .json();
        .pretty();

    let writer = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .max_log_files(cfg.max_log_files)
        .filename_prefix(&cfg.log_file_prefix)
        .filename_suffix(&cfg.log_file_suffix)
        .build(&cfg.log_dir)
        .with_context(|| format!("Failed to build log writer\ncfg: {:#?}", &cfg))?;
    let (writer, writer_guard) = non_blocking(writer);
    Box::leak(Box::new(writer_guard));

    let filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(&cfg.log_level));

    let main_layer = layer()
        .event_format(formatter)
        // .fmt_fields(JsonFields::new())
        .with_writer(writer)
        .with_filter(filter);

    registry().with(main_layer).init();
    Ok(())
}
