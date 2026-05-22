use crate::types::OrangeColor;
use colored::*;
use log::debug;
use open;

use crate::version::print_version;
// use std::env;
// use std::fs;

// const LOCAL_HELPER: &str = include_str!("../helper.md");
const LINK_HELPER: &str =
    "https://github.com/RenanZX/root-control-mapper/blob/main/readme/helper.md";

pub fn open_helper() {
    open_web();
}

pub fn open_cmds() {
    let cmds = [
        "-v - Print Version".cyan(),
        "-b - Run app in background".yellow(),
        "-k - Kill instance of app running in background".red(),
        "-j - Show JSON maps folder location".orange(),
        "-s - Show Scripts folder location".orange(),
        "-h - Open Helper of settings and maps".blue(),
        "-hc - Open CLI Helper".blue(),
    ];
    print_version();
    for cmd in cmds {
        println!("{}", cmd);
    }
}

fn open_web() {
    match open::that(LINK_HELPER) {
        Ok(_) => (),
        Err(e) => debug!("Error opening helper: {}", e),
    }
}

// fn open_local() {
//     let mut caminho_temp = env::temp_dir();
//     caminho_temp.push("rootctrlmapper_help.md");
//
//     // Escreve o texto de ajuda no arquivo temporário
//     if fs::write(&caminho_temp, LOCAL_HELPER).is_ok() {
//         // Abre o arquivo temporário com o leitor de Markdown padrão do sistema
//         match open::that(&caminho_temp) {
//             Ok(_) => (),
//             Err(e) => debug!("Error opening helper: {}", e),
//         }
//     }
// }
