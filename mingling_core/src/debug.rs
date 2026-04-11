#[macro_export]
macro_rules! only_debug {
    ($($expr:expr);* $(;)?) => {
        #[cfg(feature = "debug")]
        {
            $($expr;)*
        }
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::only_debug! {
            log::trace!($($arg)*)
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::only_debug! {
            log::debug!($($arg)*)
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::only_debug! {
            log::info!($($arg)*)
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::only_debug! {
            log::warn!($($arg)*)
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::only_debug! {
            log::error!($($arg)*)
        }
    };
}

#[cfg(feature = "debug")]
pub fn init_env_logger() {
    let mut log_path = std::env::current_exe()
        .expect("Failed to get current executable path")
        .parent()
        .expect("Failed to get parent directory")
        .to_path_buf();
    // Search for _log.txt in parent directories
    let mut current_dir = log_path.parent().unwrap().to_path_buf();
    let mut found_log = false;

    while current_dir.parent().is_some() {
        let log_file = current_dir.join("_log.txt");
        if log_file.exists() {
            log_path = log_file;
            found_log = true;
            break;
        }
        current_dir = current_dir.parent().unwrap().to_path_buf();
    }

    // If not found, use "log.txt" in the original location
    if !found_log {
        log_path = std::env::current_exe()
            .expect("Failed to get current executable path")
            .parent()
            .expect("Failed to get parent directory")
            .to_path_buf();
        log_path.push("log.txt");
    }

    // Only initialize logger if log file exists
    if log_path.exists() {
        use env_logger::Target;
        use log::LevelFilter;
        use std::fs::OpenOptions;

        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .expect("Failed to open log file");

        env_logger::Builder::new()
            .filter_level(LevelFilter::Trace)
            .target(Target::Pipe(Box::new(log_file)))
            .init();
    }
}
