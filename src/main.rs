mod app;
mod controller;
mod conversor;
mod helper;
mod interpreter;
mod leitor;
mod processor;
mod types;
mod version;

use crate::{
    app::{get_options, run_app, run_background, stop_background},
    helper::{open_cmds, open_helper},
    version::print_version,
};

fn main() {
    if let Some(option) = get_options() {
        match option.as_str() {
            "-k" => stop_background(),
            "-b" => run_background(),
            "-h" => open_helper(),
            "-v" => print_version(),
            "-hc" => open_cmds(),
            _ => run_app(),
        }
    } else {
        run_app();
    }
}
