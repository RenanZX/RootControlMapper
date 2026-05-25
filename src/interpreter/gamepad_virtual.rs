use crate::interpreter::gamepad_utils::get_virtual_player;
use crate::types::AppMode;
use colored::*;
use evdev::uinput::{VirtualDevice, VirtualDeviceBuilder};
use evdev::{AbsInfo, AbsoluteAxisType, AttributeSet, Key, UinputAbsSetup};
use evdev::{Device, InputId};
use gilrs::{GamepadId, Gilrs};
use log::debug;
use std::io;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

pub const VIRTUAL_GAMEPAD: &str = "Xbox Mouse Mode";
static ACTIVE_CONTROLLER: Mutex<Option<Arc<RwLock<AppMode>>>> = Mutex::new(None);

fn find_device() -> Option<Device> {
    for (_, device) in evdev::enumerate() {
        // Forma 1: Buscar pelo nome do dispositivo
        // println!("Searching device...");
        if let Some(name) = device.name() {
            // println!("device name: {}", name);
            // Ajuste o nome conforme o seu controle (ex: "Xbox", "DualSense", "Wireless Controller")
            let devicename = name.to_lowercase();
            if (devicename.contains("generic x-box") || devicename.contains("xbox"))
                && !devicename.contains("keyboard")
            {
                debug!("Found device: {}", name);
                return Some(device);
            }
        }
    }

    None
}

fn check_device() -> bool {
    for (_, device) in evdev::enumerate() {
        // Forma 1: Buscar pelo nome do dispositivo
        // println!("Searching device...");
        if let Some(name) = device.name() {
            // println!("device name: {}", name);
            // Ajuste o nome conforme o seu controle (ex: "Xbox", "DualSense", "Wireless Controller")
            let devicename = name.to_lowercase();
            if devicename.contains("xbox mouse mode") {
                debug!("Found Device: {}", name);
                return true;
            }
        }
    }
    return false;
}

fn build_device_controller() -> Result<VirtualDevice, io::Error> {
    let mut keys = AttributeSet::<Key>::new();
    let id = InputId::new(
        evdev::BusType::BUS_USB,
        0x2934, //id Vendor
        0x5690, //idProduct
        0,
    );

    // Botões de Ação principais
    keys.insert(Key::BTN_SOUTH); // Botão A
    keys.insert(Key::BTN_EAST); // Botão B
    keys.insert(Key::BTN_NORTH); // Botão X
    keys.insert(Key::BTN_WEST); // Botão Y

    // Bumpers (Ombro)
    keys.insert(Key::BTN_TL); // LB (Top Left)
    keys.insert(Key::BTN_TR); // RB (Top Right)

    // Botões do Centro
    keys.insert(Key::BTN_SELECT); // Botão "View" (Dois quadradinhos / Back)
    keys.insert(Key::BTN_START); // Botão "Menu" (Três linhas / Start)
    keys.insert(Key::BTN_MODE); // Botão Logo Xbox (Guia)

    // Cliques dos Analógicos
    keys.insert(Key::BTN_THUMBL); // Clique no Analógico Esquerdo (LS)
    keys.insert(Key::BTN_THUMBR); // Clique no Analógico Direito (RS)
    // 1. Configura as informações padrão para os analógicos principais (X, Y, RX, RY)
    let analog_info = AbsInfo::new(0, -32768, 32767, 16, 128, 0);
    // 2. Configura as informações para os Gatilhos (Z = LT, RZ = RT)
    let trigger_info = AbsInfo::new(0, 0, 1023, 0, 0, 0);
    // 3. Configura as informações para o D-Pad (Hat0X = Esquerda/Direita, Hat0Y = Cima/Baixo)
    let dpad_info = AbsInfo::new(0, -1, 1, 0, 0, 0);

    let virtual_dev = VirtualDeviceBuilder::new()?
        .name(VIRTUAL_GAMEPAD)
        .with_keys(&keys)?
        .input_id(id)
        .with_absolute_axis(&UinputAbsSetup::new(AbsoluteAxisType::ABS_X, analog_info))?
        .with_absolute_axis(&UinputAbsSetup::new(AbsoluteAxisType::ABS_Y, analog_info))?
        .with_absolute_axis(&UinputAbsSetup::new(AbsoluteAxisType::ABS_RX, analog_info))?
        .with_absolute_axis(&UinputAbsSetup::new(AbsoluteAxisType::ABS_RY, analog_info))?
        .with_absolute_axis(&UinputAbsSetup::new(AbsoluteAxisType::ABS_Z, trigger_info))?
        .with_absolute_axis(&UinputAbsSetup::new(AbsoluteAxisType::ABS_RZ, trigger_info))?
        .with_absolute_axis(&UinputAbsSetup::new(AbsoluteAxisType::ABS_HAT0X, dpad_info))?
        .with_absolute_axis(&UinputAbsSetup::new(AbsoluteAxisType::ABS_HAT0Y, dpad_info))?
        .build()?;
    Ok(virtual_dev)
}

fn build_virtual_device(mode: Arc<RwLock<AppMode>>) {
    match find_device() {
        Some(mut controller) => {
            controller.grab().unwrap();
            // println!("Grab input controller");
            let mut virtual_dev = build_device_controller().unwrap();
            loop {
                {
                    let actual_mode = mode.read().unwrap();
                    if *actual_mode == AppMode::GameMode {
                        break; // Sai do loop e limpa tudo (ungrab + deleta virtual)
                    }
                }
                match controller.fetch_events() {
                    Ok(iterator) => {
                        for event in iterator {
                            virtual_dev.emit(&[event]).unwrap();
                        }
                    }
                    Err(_) => {
                        debug!("[DISCONNECTED] Main Controller");
                        break;
                    }
                }
            }
        }
        None => (),
    }
    let mut active = ACTIVE_CONTROLLER.lock().unwrap();
    *active = None;
}

pub fn create_virtual_controller(
    gilrs: &mut Gilrs,
    pid: GamepadId,
    mode: AppMode,
) -> (GamepadId, Arc<RwLock<AppMode>>) {
    let mut player1_id = pid;

    // Verifica se já existe uma sessão ativa funcional
    {
        let active = ACTIVE_CONTROLLER.lock().unwrap();
        if active.is_some() && check_device() {
            if let Some(id) = get_virtual_player(gilrs) {
                player1_id = id;
            }
            return (player1_id, Arc::clone(active.as_ref().unwrap()));
        }
    }

    println!(
        "{}",
        "Preparing device due to connection/reconnection...".yellow()
    );

    let mode_shared = Arc::new(RwLock::new(mode));
    let mode_thread = Arc::clone(&mode_shared);

    // Salva a nova referência na global
    {
        let mut active = ACTIVE_CONTROLLER.lock().unwrap();
        *active = Some(Arc::clone(&mode_shared));
    }

    // Cria a nova thread para o controle que acabou de ser plugado
    thread::spawn(move || {
        build_virtual_device(mode_thread);
    });

    thread::sleep(Duration::from_secs(2));
    while gilrs.next_event().is_some() {}

    if let Some(id) = get_virtual_player(gilrs) {
        player1_id = id;
    } else {
        debug!("Error: Not found controller");
    }

    println!("{}", "Mouse Mode is ready!".green());
    (player1_id, mode_shared)
}
