use crate::interpreter;
use crate::interpreter::vk::vk_ui::render_vk;
use crate::leitor;
use crate::types::AppMode;
use crate::version::print_version;

use colored::*;
use evdev::Key;

use log::debug;
use std::env;
use std::fs;
use std::process;
const PID_FILE: &str = "/tmp/pid_root_ctrlmapper.pid";
// use log::{debug, error, info, trace, warn};

pub fn get_options() -> Option<String> {
    env::args().nth(1)
}

pub fn run_background() {
    if fs::metadata(PID_FILE).is_ok() {
        println!("{}", "App already started in background...".yellow());
        return;
    }

    unsafe {
        // O primeiro argumento (1) mantém o diretório atual.
        // O segundo argumento (0) redireciona a saída (stdout/stderr) para /dev/null
        if libc::daemon(1, 0) < 0 {
            eprintln!("Failed to start App in background.");
            process::exit(1);
        }
    }

    let pid = process::id();
    fs::write(PID_FILE, pid.to_string()).expect("Failed to save PID file");
    run_app();
}

pub fn stop_background() {
    // 1. Lê o PID salvo no arquivo
    let pid_str = match fs::read_to_string(PID_FILE) {
        Ok(conteudo) => conteudo,
        Err(_) => {
            println!(
                "{}",
                "Error: App is not running (PID file not found)."
                    .red()
                    .bold()
            );
            return;
        }
    };

    let pid: i32 = pid_str.trim().parse().expect("Invalid PID in file.");

    // 2. Envia o sinal de término (SIGTERM) para o processo Linux
    unsafe {
        if libc::kill(pid, libc::SIGTERM) == 0 {
            debug!("Stop signal sent to process {}.", pid);
            // Remove o arquivo de PID
            let _ = fs::remove_file(PID_FILE);
        } else {
            debug!(
                "Failed to stop process {}. It may have already been terminated.",
                pid
            );
            let _ = fs::remove_file(PID_FILE);
        }
    }
}

pub fn run_app() {
    env_logger::init();
    let mut mode: AppMode = AppMode::GameMode;
    let mut clipboard: Option<Vec<Key>> = None;
    print_version();
    loop {
        match mode {
            AppMode::GameMode => {
                let dados_json = leitor::get_data(mode).expect("Erro ao ler JSON");
                leitor::print_data(&dados_json);

                println!("{}", "Starting Game Mode...".blue().bold());
                if let Some(new_mode) = interpreter::macroscript::read_input_controller(dados_json)
                {
                    mode = new_mode;
                } else {
                    break;
                }
            }
            AppMode::MouseMode => {
                let dados_json = leitor::get_data(mode).expect("Erro ao ler JSON");
                leitor::print_data(&dados_json);
                println!("{}", "Starting Mouse Mode...".yellow().bold());

                if let Some(new_mode) =
                    interpreter::mousekeyboard::read_input_controller(dados_json, &clipboard)
                {
                    mode = new_mode;
                } else {
                    break;
                }
            }
            AppMode::KeyboardMode => {
                println!("{}", "Opening Keyboard...".purple().bold());
                let (new_mode, clipcmds) = render_vk();
                if new_mode == AppMode::MouseMode {
                    mode = new_mode;
                    clipboard = clipcmds;
                } else {
                    break;
                }
            }
        }
    }
    //println!("Dados do JSON: {:?}", dados_json);
}
