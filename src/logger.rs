use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;
use std::io::BufWriter;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Log(#[from] log::SetLoggerError),
}

pub fn initialize() -> Result<(), Error> {
    const LOG_LEVEL: LevelFilter = LevelFilter::Debug;

    let terminal = TermLogger::new(LOG_LEVEL, Config::default(), TerminalMode::Mixed);

    let file = File::create("kf1.log").map(BufWriter::new)?;
    let file = WriteLogger::new(LOG_LEVEL, Config::default(), file);

    CombinedLogger::init(vec![terminal, file])?;

    Ok(())
}
