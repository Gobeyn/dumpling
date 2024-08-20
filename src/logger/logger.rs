use env_logger::Builder;
use std::io::Write;

/// Create $HOME/.cache/dumpling/dumpling.log file and set up the logger.
pub fn init_logging() {
    // Create file path to the logger
    let mut log_file_path = dirs::cache_dir().expect("Error obtaining $HOME/.cache");
    log_file_path.push("dumpling");
    log_file_path.push("dumpling.log");

    // Create and open log file.
    let log_file = std::fs::File::create(log_file_path).expect("Error creating log file");
    // Only one thread allowed.
    let log_file = std::sync::Mutex::new(log_file);

    // Set up the logger
    Builder::new()
        .format(move |_buf, record| {
            let mut log_file = log_file.lock().unwrap();
            writeln!(
                log_file,
                "{} [{}] - {}:{} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Info)
        .init();
}
