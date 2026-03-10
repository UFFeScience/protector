use log::LevelFilter;

pub(crate) fn init(level: LevelFilter) -> Result<(), anyhow::Error> {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {}: {}",
                chrono::Local::now().format("[%H:%M:%S%.3f]"),
                record.level(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

pub fn log_level(verbosity: u8) -> LevelFilter {
    match verbosity {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    }
}
