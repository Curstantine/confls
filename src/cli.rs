use clap::{arg, command, Parser};

#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(long, short)]
    pub name: String,
    #[arg(long, default_value = "../environments")]
    pub environment_dir: String,
    #[arg(long, default_value_t = true)]
    pub no_root: bool,
}

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
        $crate::cli::log($crate::cli::LogType::$log_type, &format!($($arg)*));
    };
    () => {};
}
