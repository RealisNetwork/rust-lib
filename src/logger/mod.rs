//! Provides logging utilities, used by application.

pub mod level;
pub mod nats;

use colored::Colorize;
use std::io;

use chrono::{self, Local};
use log::{Level as LogLevel, LevelFilter};
use slog::{o, Drain, Duplicate, FnValue, Fuse, Level, Logger, PushFnValue, Record};
use slog_async::Async;
#[allow(unused_imports)]
use slog_json::Json;
use slog_term::{CompactFormat, PlainSyncDecorator};

pub mod prelude {
    pub use slog::{slog_debug, slog_error, slog_info, slog_trace, slog_warn};
    pub use slog_scope::{debug, error, info, trace, warn};
}

pub fn new<W1, W2>(w_out: W1, w_err: W2) -> Logger
where
    W1: io::Write + Send + 'static,
    W2: io::Write + Send + 'static,
{
    // let drain_out = Json::new(w_out).build();
    // let drain_err = Json::new(w_err).build();

    let drain_out = CompactFormat::new(PlainSyncDecorator::new(w_out)).build();
    let drain_err = CompactFormat::new(PlainSyncDecorator::new(w_err)).build();

    let drain = Duplicate(
        drain_out.filter_level(Level::Info),
        drain_err.filter_level(Level::Error),
    )
    .map(Fuse);

    // let drain = slog_envlogger::new(drain).fuse();
    let drain = Async::new(drain).chan_size(2048).build().fuse();
    let logger = Logger::root(drain, o!());

    logger.new(o!(
        "lvl" => FnValue(move |rinfo : &Record| {
            // TODO not all colored
            match rinfo.level() {
                Level::Critical
                | Level::Error => rinfo.level().as_str().red().to_string(),
                Level::Warning => rinfo.level().as_str().yellow().to_string(),
                Level::Info => rinfo.level().as_str().blue().to_string(),
                Level::Debug => rinfo.level().as_str().green().to_string(),
                Level::Trace => rinfo.level().as_str().magenta().to_string(),
            }
        }),
        "msg" => PushFnValue(move |record : &Record, ser| {
            ser.emit(record.msg())
        }),
        "fqn" => PushFnValue(move |record : &Record, ser| {
             ser.emit(format_args!("{}:{}", record.module(), record.line()))
        }),
        "time" => PushFnValue(move |_ : &Record, ser| {
            ser.emit(Local::now().to_rfc3339())
        }),
    ))
}

#[allow(dead_code)]
pub fn logger_init(level: LevelFilter) {
    use env_logger::Builder;
    use std::io::Write;

    let _ = Builder::new()
        .format(|buf, record| match record.level() {
            LogLevel::Error => writeln!(
                buf,
                "[{}] - {}",
                format!("{}", record.level()).red(),
                record.args()
            ),
            LogLevel::Warn => writeln!(
                buf,
                "[{}]  - {}",
                format!("{}", record.level()).yellow(),
                record.args()
            ),
            LogLevel::Info => writeln!(
                buf,
                "[{}]  - {}",
                format!("{}", record.level()).blue(),
                record.args()
            ),
            LogLevel::Debug => writeln!(
                buf,
                "[{}] - {}",
                format!("{}", record.level()).green(),
                record.args()
            ),
            LogLevel::Trace => writeln!(
                buf,
                "[{}] - {}",
                format!("{}", record.level()).magenta(),
                record.args()
            ),
        })
        .filter(None, level)
        .try_init();
}

#[must_use]
pub fn term_new() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let drain = drain.filter_level(Level::Info).fuse();

    let logger = slog::Logger::root(drain, o!());

    logger.new(o!(
        "msg" => PushFnValue(move |record : &Record, ser| {
            ser.emit(record.msg())
        }),
        "fqn" => PushFnValue(move |record : &Record, ser| {
             ser.emit(format_args!("{:?}, {}:{}", chrono::Utc::now(),
                record.module(), record.line()))
        }),
        "time" => PushFnValue(move |_ : &Record, ser| {
            ser.emit(Local::now().to_rfc3339())
        }),
        "lvl" => FnValue(move |rinfo : &Record| {
            rinfo.level().as_str()
        }),
    ))
}
