use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

use crate::settings::RawImportLogLevel;
use crate::RawImportArgs;

pub(crate) fn setup_logging(args: &RawImportArgs) -> anyhow::Result<log4rs::Handle> {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}{n}")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(match args.verbosity {
            RawImportLogLevel::Info => LevelFilter::Info,
            RawImportLogLevel::Debug => LevelFilter::Debug,
            RawImportLogLevel::Trace => LevelFilter::Trace,
        }))?;

    let handle = log4rs::init_config(config)?;
    Ok(handle)
}
