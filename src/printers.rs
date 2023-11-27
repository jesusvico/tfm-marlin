pub use colored::Colorize;

pub fn print_panic(info: &str) {
    panic!("{} {}",
    "Error:".red().bold(),
    info)
}

#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*) => {
        
        println!(
            "{} {}",
            colored::Colorize::bold(colored::Colorize::yellow("Info:")),
            format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! print_panic {
    ($($arg:tt)*) => {

        panic!(
            "{} {}",
            colored::Colorize::bold(colored::Colorize::red("Error:")),
            format_args!($($arg)*)
        )
    };
}
