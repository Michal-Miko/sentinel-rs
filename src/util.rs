use std::io::stdout;

use color_eyre::Result;
use tracing::trace;
use tracing_appender::rolling::daily;
use tracing_subscriber::fmt::writer::MakeWriterExt;

const LOG_PREFIX: &str = "sentinel-rs";

#[expect(dead_code)]
#[derive(Clone, Copy)]
pub enum LogConfig<'src> {
    Stdout {
        level: tracing::Level,
    },
    File {
        level: tracing::Level,
        log_directory: &'src str,
    },
    Combined {
        stdout_level: tracing::Level,
        file_level: tracing::Level,
        log_directory: &'src str,
    },
}

impl<'src> Default for LogConfig<'src> {
    fn default() -> Self {
        Self::Combined {
            stdout_level: tracing::Level::INFO,
            file_level: tracing::Level::TRACE,
            log_directory: "logs",
        }
    }
}

fn build_base_subscriber() -> tracing_subscriber::fmt::SubscriberBuilder {
    tracing_subscriber::fmt().with_max_level(tracing::Level::TRACE)
}

fn setup_stdout(level: tracing::Level) {
    build_base_subscriber()
        .with_writer(stdout.with_max_level(level))
        .init();
}

fn setup_file(level: tracing::Level, log_directory: &str) {
    build_base_subscriber()
        .with_writer(daily(log_directory, LOG_PREFIX).with_max_level(level))
        .init();
}

fn setup_combined(stdout_level: tracing::Level, file_level: tracing::Level, log_directory: &str) {
    build_base_subscriber()
        .with_writer(
            stdout
                .with_max_level(stdout_level)
                .and(daily(log_directory, LOG_PREFIX).with_max_level(file_level)),
        )
        .init();
}

pub fn setup_tracing(config: LogConfig) -> Result<()> {
    color_eyre::install()?;

    match config {
        LogConfig::Stdout { level } => setup_stdout(level),

        LogConfig::File {
            level,
            log_directory,
        } => setup_file(level, log_directory),

        LogConfig::Combined {
            stdout_level,
            file_level,
            log_directory,
        } => setup_combined(stdout_level, file_level, log_directory),
    };

    trace!("Tracing set up successfully.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::util::{setup_tracing, LogConfig};

    #[test]
    fn default_log_config_works() {
        assert!(setup_tracing(LogConfig::default()).is_ok());
    }
}
