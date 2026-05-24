use colored::*;
use evdev::RelativeAxisType;
use evdev::uinput::{VirtualDevice, VirtualDeviceBuilder};
use evdev::{AttributeSet, Key};
use gilrs::{GamepadId, Gilrs};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::gamepad_virtual::VIRTUAL_GAMEPAD;
use crate::controller::keyboard_controller;
use crate::controller::keyboard_controller::KeyboardXbxController;
use crate::leitor;
use crate::types::{AppAction, MapCmd};

pub fn get_player1(gilrs: &mut Gilrs) -> Option<GamepadId> {
    gilrs.gamepads().next().map(|(id, _)| id)
}

pub fn get_virtual_player(gilrs: &mut Gilrs) -> Option<GamepadId> {
    gilrs
        .gamepads()
        .find(|(_, gamepad)| gamepad.name() == VIRTUAL_GAMEPAD)
        .map(|(id, _)| id)
}

pub fn print_gamepad_status(player_id: &Option<GamepadId>, gilrs: &mut Gilrs) {
    let mut controller_status = "[DISCONNECTED]".to_string().red().bold();
    if let Some(id) = player_id {
        let name = gilrs.gamepad(*id).name().to_string();
        controller_status = format!("[CONNECTED] {}", name).green().bold();
    }
    println!("{}", controller_status);
}

pub fn check_gamepad(
    gilrs: &mut Gilrs,
    data_map: &Vec<MapCmd>,
) -> (GamepadId, Option<Arc<Mutex<KeyboardXbxController>>>) {
    println!("{}", "Waiting for controller to connect...".yellow());

    loop {
        // Drena eventos pendentes para capturar Connected antes de tentar get_player1
        while let Some(_) = gilrs.next_event() {}

        if let Some(player_id) = get_player1(gilrs) {
            print_gamepad_status(&Some(player_id), gilrs);

            let xbox_keyboard_option: Option<Arc<Mutex<KeyboardXbxController>>> =
                if keyboard_controller::check_device() && leitor::contains_rec(data_map) {
                    Some(Arc::new(Mutex::new(
                        KeyboardXbxController::create().unwrap(),
                    )))
                } else {
                    None
                };

            if let Some(ref kbd_arc) = xbox_keyboard_option {
                let kbd_clone = Arc::clone(kbd_arc);
                thread::spawn(move || {
                    loop {
                        {
                            let mut kbd = kbd_clone.lock().unwrap();
                            kbd.update_input();
                        }
                        thread::sleep(Duration::from_millis(10));
                    }
                });
            }

            return (player_id, xbox_keyboard_option);
        }

        thread::sleep(Duration::from_millis(500));
    }
}

fn get_all_keys(data_map: &Vec<MapCmd>) -> Vec<Key> {
    data_map
        .iter()
        .filter_map(|cmd| {
            // Extrai o vetor de macros se a ação for MacroKeys
            if let Some(AppAction::MacroKeys(macros)) = &cmd.action {
                Some(macros)
            } else {
                None
            }
        })
        .flatten() // Transforma o Iterador de &Vec<Key> em um iterador de itens &Key individuais
        .copied() // Transforma &Key em Key (copia os valores). Requer que 'Key' seja Copy.
        .collect() // Junta tudo no Vec<Key> final
}

pub fn build_kb_device(data_map: &Vec<MapCmd>) -> VirtualDevice {
    let keys: Vec<Key> = get_all_keys(data_map);

    // 1. Ativar todas as teclas no dispositivo virtual
    let mut keys_to_enable = AttributeSet::<Key>::new();
    for k in &keys {
        keys_to_enable.insert(*k);
    }

    VirtualDeviceBuilder::new()
        .unwrap()
        .name("MacroVirtualKeyboard")
        .with_keys(&keys_to_enable)
        .unwrap()
        .build()
        .unwrap()
}

pub fn build_kb_device_mouse() -> VirtualDevice {
    let keys_to_enable = AttributeSet::<Key>::from_iter((0..560).map(Key));

    let mut axes = AttributeSet::<RelativeAxisType>::new();
    axes.insert(RelativeAxisType::REL_X);
    axes.insert(RelativeAxisType::REL_Y);
    axes.insert(RelativeAxisType::REL_WHEEL);
    axes.insert(RelativeAxisType::REL_HWHEEL);

    VirtualDeviceBuilder::new()
        .expect("Falha ao iniciar Builder")
        .name("VirtualMouseKeyboard")
        .with_keys(&keys_to_enable)
        .expect("Falha ao registrar chaves")
        .with_relative_axes(&axes)
        .expect("Falha ao registrar eixos")
        .build()
        .expect("Falha ao criar dispositivo")
}
