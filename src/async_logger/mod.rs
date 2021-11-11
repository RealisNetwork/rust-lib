pub mod nats;
pub mod level;

use slog::{o, Drain, Logger, OwnedKVList, Record, PushFnValue, FnValue, Level};
use slog_async::Async;
use slog_envlogger::LogBuilder;
use slog_scope::GlobalLoggerGuard;
use slog_term::{Decorator, TermDecorator};
use colored::Colorize;
use chrono::Local;
use log::LevelFilter;

/// The channel size for async logging.
const BUFFER_SIZE: usize = 1024;

/// Initialize driver logging.
pub fn init(level: LevelFilter) -> (Logger, GlobalLoggerGuard) {
    // Log errors to stderr and lower severities to stdout.
    let format = CustomFormatter::new(
        TermDecorator::new().stderr().build(),
        TermDecorator::new().stdout().build(),
    )
        .fuse();
    let drain = Async::new(LogBuilder::new(format).parse(&level.to_string()).build())
        .chan_size(BUFFER_SIZE)
        .build();
    let logger = Logger::root(drain.fuse(), o!());

    let guard = slog_scope::set_global_logger(logger.clone());
    // slog_stdlog::init().expect("failed to register logger");

    (logger.new(o!(
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
    )), guard)
}

/// Uses one decorator for `Error` and `Critical` log messages and the other for
/// the rest.
pub struct CustomFormatter<ErrDecorator, RestDecorator> {
    err_decorator: ErrDecorator,
    rest_decorator: RestDecorator,
}

impl<ErrDecorator, RestDecorator> CustomFormatter<ErrDecorator, RestDecorator> {
    fn new(err_decorator: ErrDecorator, rest_decorator: RestDecorator) -> Self {
        Self {
            err_decorator,
            rest_decorator,
        }
    }
}

impl<ErrDecorator: Decorator, RestDecorator: Decorator> Drain
for CustomFormatter<ErrDecorator, RestDecorator>
{
    type Ok = ();
    type Err = std::io::Error;
    fn log(
        &self,
        record: &Record,
        values: &OwnedKVList,
    ) -> std::result::Result<Self::Ok, Self::Err> {
        match record.level() {
            Level::Error | Level::Critical => log_to_decorator(&self.err_decorator, record, values),
            _ => log_to_decorator(&self.rest_decorator, record, values),
        }
    }
}

fn log_to_decorator(
    decorator: &impl Decorator,
    record: &Record,
    values: &OwnedKVList,
) -> std::result::Result<(), std::io::Error> {
    decorator.with_record(record, values, |mut decorator| {
        decorator.start_timestamp()?;
        slog_term::timestamp_utc(&mut decorator)?;

        decorator.start_whitespace()?;
        write!(decorator, " ")?;

        decorator.start_level()?;
        write!(decorator, "{}", record.level())?;

        decorator.start_whitespace()?;
        write!(decorator, " ")?;

        write!(decorator, "[{}]", record.module())?;

        decorator.start_whitespace()?;
        write!(decorator, " ")?;

        decorator.start_msg()?;
        writeln!(decorator, "{}", record.msg())?;

        decorator.flush()?;

        Ok(())
    })
}