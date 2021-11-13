use colored::Colorize;
use slog::Level;
use slog::{o, Drain, Logger, OwnedKVList, Record};
use slog_async::Async;
use slog_envlogger::LogBuilder;
use slog_scope::GlobalLoggerGuard;
use slog_term::{Decorator, TermDecorator};

/// The channel size for async logging.
const BUFFER_SIZE: usize = 2048;

/// Initialize driver logging.
pub fn init(filter: impl AsRef<str>) -> (Logger, GlobalLoggerGuard) {
    // Log errors to stderr and lower severities to stdout.
    let format = CustomFormatter::new(
        TermDecorator::new().stderr().build(),
        TermDecorator::new().stdout().build(),
    )
        .fuse();
    let drain = Async::new(LogBuilder::new(format).parse(filter.as_ref()).build())
        .chan_size(BUFFER_SIZE)
        .build();
    let logger = Logger::root(drain.fuse(), o!());

    let guard = slog_scope::set_global_logger(logger.clone());
    slog_stdlog::init().expect("failed to register logger");

    (logger, guard)
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

        match record.level() {
            Level::Critical |
            Level::Error => writeln!(
                decorator,
                "[{}] - [{}] - {} - {}",
                format!("{}", record.level()).red(),
                record.module().red(),
                record.line(),
                record.msg(),
            ),
            Level::Warning => writeln!(
                decorator,
                "[{}]  - [{}] - {}",
                format!("{}", record.level()).yellow(),
                record.module().yellow(),
                record.msg(),
            ),
            Level::Info => writeln!(
                decorator,
                "[{}]  - [{}] - {}",
                format!("{}", record.level()).blue(),
                record.module().blue(),
                record.msg(),
            ),
            Level::Debug => writeln!(
                decorator,
                "[{}] - [{}] - {}",
                format!("{}", record.level()).green(),
                record.module().green(),
                record.msg(),
            ),
            Level::Trace => writeln!(
                decorator,
                "[{}] - [{}] - {} - {}",
                format!("{}", record.level()).magenta(),
                record.module().magenta(),
                record.line(),
                record.msg(),
            ),
        };
        decorator.flush()?;

        Ok(())
    })
}