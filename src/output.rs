use std::fmt::Display;

pub fn print_kv<T: Display>(label: &str, value: T) {
    println!("{:<22} {}", label, value);
}

pub fn print_title(title: &str) {
    println!("\n{}\n{}", title, "-".repeat(title.len()));
}

pub fn print_header(label: &str) {
    println!("\n{}", label);
}