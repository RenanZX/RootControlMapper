use colored::*;

pub fn get_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    version.to_string()
}

pub fn print_version() {
    let app_v = format!("Root Control Mapper {}", get_version());
    println!("{}", app_v.cyan().bold());
}
