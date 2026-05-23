use colored::*;
use evdev::uinput::VirtualDevice;
use evdev::{EventType, InputEvent, Key, RelativeAxisType};
use log::{debug, error};
use std::env;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

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

pub fn exec_macro(keys: &Vec<Key>, device: &mut VirtualDevice) -> Option<Key> {
    debug!("{}", format!("Executar macro: {:?}", keys).purple());
    if keys.len() == 1 {
        let key = keys[0];
        if is_scroll_key(key) {
            tap_scroll(key, device);
        } else {
            press_key(&key, device);
        }

        return Some(keys[0]);
    } else {
        press_mult_key(keys, device);
        return None;
    }
}

pub fn paste_clipboard(device: &mut VirtualDevice) {
    press_mult_key(&vec![Key::KEY_LEFTCTRL, Key::KEY_V], device);
    debug!("Texto colado!");
}

fn press_mult_key(keys: &Vec<Key>, device: &mut VirtualDevice) {
    let delay = Duration::from_millis(50);

    // 3. Pressionar todas (Ex: ALT + T)
    for k in keys {
        debug!("Pressionado tecla {:?}", k.code());
        let ev = InputEvent::new(EventType::KEY, k.code(), 1); // 1 = Press
        device.emit(&[ev]).unwrap();
        thread::sleep(delay);
    }

    // Espera mínima com tudo pressionado para o atalho ser reconhecido
    thread::sleep(Duration::from_millis(100));

    // 4. Soltar todas na ordem inversa (Ex: T e depois ALT)
    for k in keys.iter().rev() {
        let ev = InputEvent::new(EventType::KEY, k.code(), 0); // 0 = Release[cite: 2]
        device.emit(&[ev]).unwrap();
        thread::sleep(delay);
    }
}

pub fn press_key(k: &Key, device: &mut VirtualDevice) {
    let delay = Duration::from_millis(50);

    let ev = InputEvent::new(EventType::KEY, k.code(), 1); // 1 = Press
    device.emit(&[ev]).unwrap();
    thread::sleep(delay);
}

pub fn release_key(k: &Key, device: &mut VirtualDevice) {
    let delay = Duration::from_millis(50);

    let ev = InputEvent::new(EventType::KEY, k.code(), 0); // 0 = Release[cite: 2]
    device.emit(&[ev]).unwrap();
    thread::sleep(delay);
}

fn scroll_mouse(clicks: i32, horizontal: bool, device: &mut VirtualDevice) {
    let delay = Duration::from_millis(50);

    // 1. Determina qual eixo usar baseado na direção informada
    let axis = if horizontal {
        RelativeAxisType::REL_HWHEEL.0
    } else {
        RelativeAxisType::REL_WHEEL.0
    };

    // 2. Envia os pulsos de rolagem um por um para simular o comportamento físico
    let step = if clicks > 0 { 1 } else { -1 };
    let total_steps = clicks.abs();

    for _ in 0..total_steps {
        debug!(
            "Executando pulso de scroll no eixo {} com valor {}",
            axis, step
        );

        let ev = InputEvent::new(EventType::RELATIVE, axis, step);
        device
            .emit(&[ev])
            .expect("Falha ao emitir evento de scroll");

        thread::sleep(delay);
    }
}

fn tap_scroll(key: Key, device: &mut VirtualDevice) {
    let speed: i32 = 3;
    let horizontal = false;
    if key == Key::KEY_SCROLLUP {
        scroll_mouse(speed, horizontal, device);
    } else {
        scroll_mouse(-speed, horizontal, device);
    }
}

fn is_scroll_key(key: Key) -> bool {
    key == Key::KEY_SCROLLUP || key == Key::KEY_SCROLLDOWN
}

pub fn move_mouse(axis_lx: f32, axis_ly: f32, device: &mut VirtualDevice) {
    let delay = Duration::from_millis(4); // ~250Hz polrate
    let deadzone = 0.15;
    let sensitivity = 55.0;
    let max_delta = 127.0;

    // Calcula a magnitude do vetor (Pitágoras)
    let magnitude = (axis_lx.powi(2) + axis_ly.powi(2)).sqrt();

    if magnitude > deadzone {
        // Normaliza a magnitude dentro do range útil [0.0, 1.0]
        let normalized_mag = (magnitude - deadzone) / (1.0 - deadzone);

        // Aplicar curva de resposta (Exponencial para precisão)
        let curve = normalized_mag.powf(1.5);

        // Calcula a direção mantendo a proporção (Círculo)
        // O eixo Y é invertido pois no mouse 'para cima' é negativo
        let dx = (axis_lx / magnitude) * curve * sensitivity;
        let dy = (-axis_ly / magnitude) * curve * sensitivity;

        // Clamp do vetor resultante para não exceder o max_delta
        let dx_final = dx.clamp(-max_delta, max_delta) as i32;
        let dy_final = dy.clamp(-max_delta, max_delta) as i32;

        if dx_final != 0 || dy_final != 0 {
            let move_x = InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_X.0, dx_final);
            let move_y = InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_Y.0, dy_final);

            if let Err(e) = device.emit(&[move_x, move_y]) {
                error!("Erro ao emitir evento: {}", e);
            }
        }
    }

    thread::sleep(delay);
}
