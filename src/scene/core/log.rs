use log::{Record, Level, Metadata, SetLoggerError,LevelFilter};

struct SimpleLogger;

impl log::Log for SimpleLogger
{
    fn enabled(&self, metadata: &Metadata) -> bool
    {
       	metadata.level() <= Level::Info    		
    }

    fn log(&self, record: &Record)
    {
        if self.enabled(record.metadata())
        {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init_log() -> Result<(), SetLoggerError>
{
    log::set_boxed_logger(Box::new(SimpleLogger))
        .map(|()| log::set_max_level(LevelFilter::Info))
}