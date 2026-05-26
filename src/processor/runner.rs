use colored::*;
// use log::{debug, error};
use std::env;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn get_local_script(script_name: &str) -> PathBuf {
    let mut local_path = env::current_exe().expect("Falha ao obter caminho do executável");
    local_path.pop();

    let script_path = if cfg!(debug_assertions) {
        local_path.pop();
        local_path.pop();
        local_path
            .join("environment/debug/scripts")
            .join(script_name)
    } else {
        local_path.join("scripts").join(script_name)
    };
    script_path
}

fn get_params(script_raw: &str) -> (&str, Vec<&str>) {
    let mut parts = script_raw.split_whitespace();
    let cmd_part = parts.next().unwrap_or("");
    (cmd_part, parts.collect())
}

fn get_py_exec() -> PathBuf {
    let mut local_path = env::current_exe().expect("Falha ao obter caminho do executável");
    local_path.pop();

    let py_exec = if cfg!(debug_assertions) {
        local_path.pop();
        local_path.pop();
        local_path.join("environment/debug/rcm_py/bin/python")
    } else {
        local_path.join("rcm_py/bin/python")
    };
    py_exec
}

fn has_path(script_path: &str) -> bool {
    Path::new(script_path).exists()
}

fn print_args(args: &Vec<&str>, type_script: &str) {
    if args.len() > 0 {
        let all_args = args.join(" ");
        if type_script == "bash" {
            println!(
                "{}",
                format!("With args: {}", all_args).truecolor(255, 100, 255)
            );
        } else {
            println!(
                "{}",
                format!("With args: {}", all_args).truecolor(180, 140, 0)
            );
        }
    }
}

pub fn run_py(py_raw: &str) {
    let (py_file, args) = get_params(py_raw);
    let script: PathBuf = if has_path(py_file) {
        PathBuf::from(py_file)
    } else {
        get_local_script(py_file)
    };
    let py_exec = get_py_exec();

    println!(
        "{}",
        format!("Run script Python {}", script.display()).yellow()
    );

    print_args(&args, "py");

    let mut child = Command::new(py_exec)
        .arg(script)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Falha ao executar o comando");

    let mut stdout = child.stdout.take().expect("Falha ao abrir stdout");
    let mut stderr = child.stderr.take().expect("Falha ao abrir stderr");

    let status = child.wait().expect("Falha ao aguardar processo");

    if status.success() {
        let mut saida = String::new();
        stdout.read_to_string(&mut saida).ok();
        println!("{} {}", "Saida:".to_string().green().bold(), saida);
    } else {
        let mut err = String::new();
        stderr.read_to_string(&mut err).ok();
        println!(
            "{} {}",
            "Erro:".to_string().red().bold(),
            err.to_string().red()
        );
    }
}

pub fn run_cmd(exec_raw: &String) {
    let (script_exec, args) = get_params(exec_raw);
    println!("script_path: {} ", script_exec);
    let script: PathBuf = if has_path(script_exec) {
        PathBuf::from(script_exec)
    } else {
        get_local_script(script_exec)
    };

    println!(
        "{}",
        format!("Run command/script: {}", script.display()).bright_magenta()
    );

    print_args(&args, "bash");

    let mut child = Command::new(script)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Falha ao executar o comando");

    let status = child.wait().expect("Falha ao aguardar processo");

    if status.success() {
        let mut saida = String::new();
        if let Some(mut stdout) = child.stdout {
            stdout.read_to_string(&mut saida).ok();
        }
        println!("{} {}", "Saida:".to_string().green().bold(), saida);
    } else {
        let mut err = String::new();
        if let Some(mut stderr) = child.stderr {
            stderr.read_to_string(&mut err).ok();
        }
        println!(
            "{} {}",
            "Erro:".to_string().red().bold(),
            err.to_string().red()
        );
    }
}
