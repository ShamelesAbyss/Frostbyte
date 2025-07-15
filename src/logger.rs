use colored::Colorize;

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        println!("{} {}", "Frostbyte 🧊🐧".green().bold(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        println!("{} {}", "Frostbyte 🧊🐧".purple().bold(), format!($($arg)*))
    };
}
