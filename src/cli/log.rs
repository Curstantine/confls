pub enum LogType {
    Info,
    Warning,
}

pub fn log(log_type: LogType, message: &str) {
    match log_type {
        LogType::Info => println!("[INFO] {:#?}", message),
        LogType::Warning => eprintln!("[WARN] {:#?}", message),
    }
}

#[macro_export]
macro_rules! log {
    ($log_type:ident, $($arg:tt)*) => {
        $crate::cli::log::log($crate::cli::log::LogType::$log_type, &format!($($arg)*));
    };
    () => {};
}
