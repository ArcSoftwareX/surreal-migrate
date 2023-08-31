use colored::Colorize;

pub fn log<T: std::fmt::Display>(val: T) {
    println!("{} {val}", " info ".on_blue().black().bold())
}

pub fn error<T: std::fmt::Display>(val: T) {
    println!("{} {val}", " error ".on_red().black().bold())
}
