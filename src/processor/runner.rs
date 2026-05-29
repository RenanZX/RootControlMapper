use super::utils::clear_folder;
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

pub fn fix_env_lua() {
    let local_path = env::current_dir().expect("Falha ao obter caminho da pasta");
    let main_path = if cfg!(debug_assertions) {
        local_path.join("environment/debug")
    } else {
        local_path.clone()
    };
    let script = main_path.join("fix/env_lua.sh");

    let lua_dir = main_path.join("rcm_lua");
    let _ = clear_folder(&lua_dir);

    println!("{}", "Fix Lua environment".to_string().blue());
    // println!("Main Dir: {}", main_path.display());
    // println!("Lua Dir: {}", lua_dir.display());

    let mut child = Command::new(script)
        .args([local_path, lua_dir])
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

fn get_lua_exec(type_exec: &str) -> PathBuf {
    let mut local_path = env::current_exe().expect("Falha ao obter caminho do executável");
    local_path.pop();
    let exec = if type_exec == "installer" {
        "pkg_lua"
    } else {
        "run_lua"
    };

    let lua_exec = if cfg!(debug_assertions) {
        local_path.pop();
        local_path.pop();
        local_path.join(format!("environment/debug/rcm_lua/{exec}.sh"))
    } else {
        local_path.join(format!("rcm_lua/{exec}.sh"))
    };
    lua_exec
}

fn has_path(script_path: &str) -> bool {
    Path::new(script_path).exists()
}

fn print_args(args: &Vec<&str>, type_script: &str) {
    if args.len() > 0 {
        let all_args = args.join(" ");
        let args_to_print = format!("With args: {}", all_args);
        match type_script {
            "bash" => println!("{}", args_to_print.truecolor(255, 100, 255)),
            "py" => println!("{}", args_to_print.truecolor(180, 140, 0)),
            "lua" => println!("{}", args_to_print.truecolor(173, 216, 230)),
            _ => (),
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

pub fn install_package_lua(lua_package_raw: &String) {
    let (lua_pkg, args) = get_params(lua_package_raw);
    println!("{}", format!("Installing lib Lua {}", lua_pkg));

    let lua_installer = get_lua_exec("installer");
    let mut child = Command::new(lua_installer)
        .arg(lua_pkg)
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

pub fn run_lua(lua_raw: &String) {
    let (lua_file, args) = get_params(lua_raw);
    let script: PathBuf = if has_path(lua_file) {
        PathBuf::from(lua_file)
    } else {
        get_local_script(lua_file)
    };
    let lua_exec = get_lua_exec("exec");

    println!("{}", format!("Run script Lua {}", script.display()).blue());

    print_args(&args, "lua");

    let mut child = Command::new(lua_exec)
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
