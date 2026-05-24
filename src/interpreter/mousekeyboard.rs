use colored::*;
use gilrs::{Axis, Button, Event, EventType, Gilrs};
use log::{debug, info};
use std::thread;
use std::time::Duration;

use super::gamepad_virtual::create_virtual_controller;
use crate::controller::button_controller::ButtonCombo;
use crate::processor;
use crate::types::InputController::Gamepad;
use crate::types::{AppAction, AppMode, MapCmd};

use crate::interpreter::gamepad_utils::*;
use evdev::Key;
use evdev::uinput::VirtualDevice;

pub fn read_input_controller(
    data_map: Vec<MapCmd>,
    clipboard: &Option<Vec<Key>>,
) -> Option<AppMode> {
    let mut gilrs = Gilrs::new().unwrap();

    // thread::sleep(Duration::from_millis(500));

    // 1. Criar o teclado virtual + mouse
    let mut device: VirtualDevice = build_kb_device_mouse();

    // Pausa crucial para o sistema operacional registrar o novo "hardware"
    thread::sleep(Duration::from_millis(300));

    let (cid, _) = check_gamepad(&mut gilrs, &data_map);

    let (mut player1_id, mut mode_shared) =
        create_virtual_controller(&mut gilrs, cid, AppMode::MouseMode);

    let mut button_control = ButtonCombo::create();

    // println!("Iniciando loop");
    let mut axis_x: f32 = 0.0;
    let mut axis_y: f32 = 0.0;
    let mut key_pressed: Option<Key> = None;

    info!("Iniciando Loop de processamento de entrada...");

    loop {
        let mut gamepad_connected = true;
        let mut botao_pressionado = false;

        // Examina novos eventos (botões, analógicos, conexão)
        while let Some(Event { id, event, .. }) = gilrs.next_event() {
            if id == player1_id {
                match event {
                    EventType::ButtonPressed(button, _) => {
                        debug!("Adicionando botao apertado! {:?}", button);
                        // Caso algum botao seja pressionado, o array sera iterado
                        button_control.add_button(Gamepad(button));
                        botao_pressionado = true;
                    }

                    EventType::ButtonReleased(_, _) => {
                        // Caso algum botao seja pressionado, o array sera iterado
                        if let Some(key) = key_pressed {
                            processor::release_key(&key, &mut device);
                            key_pressed = None;
                        }
                    }

                    EventType::AxisChanged(axis, value, _) => {
                        // Analógicos e Gatilhos (valor de -1.0 a 1.0 ou 0.0 a 1.0)
                        if let Some(move_ax) = AppAction::get_move_axis(&data_map) {
                            let type_axis_x = move_ax.x;
                            let type_axis_y = move_ax.y;
                            match axis {
                                ax if ax == type_axis_x => axis_x = value,
                                ax if ax == type_axis_y => axis_y = value,
                                _ => (),
                            };
                        }
                        if value.abs() >= 1.0 {
                            match axis {
                                Axis::LeftZ => {
                                    button_control.add_button(Gamepad(Button::LeftTrigger2));
                                    botao_pressionado = true;
                                }
                                Axis::RightZ => {
                                    button_control.add_button(Gamepad(Button::RightTrigger2));
                                    botao_pressionado = true;
                                }
                                _ => (),
                            }
                        }
                        // if value.abs() > 0.1 { // Pequena zona morta
                        //     println!("Eixo {:?} movido para: {:.2}", axis, value);
                        // }
                    }
                    EventType::Disconnected => {
                        println!("{}", "[DISCONNECTED]".red().bold());
                        gamepad_connected = false;
                    }
                    _ => (),
                }
            }
        }

        if !gamepad_connected || get_player1(&mut gilrs).is_none() {
            button_control = ButtonCombo::create();
            gilrs = Gilrs::new().unwrap();

            let (cid, _) = check_gamepad(&mut gilrs, &data_map);
            (player1_id, mode_shared) =
                create_virtual_controller(&mut gilrs, cid, AppMode::MouseMode);
            continue;
        }

        processor::move_mouse(axis_x, axis_y, &mut device);

        if botao_pressionado {
            // button_control.print_buttons();
            let value_chacked: Option<&MapCmd> = button_control.check_button_pressed(&data_map);
            if let Some(data_checked) = value_chacked {
                //println!("Value checked: {:?}", data_checked);
                if let Some(action) = &data_checked.action {
                    match action {
                        AppAction::ChangeMode => {
                            let mut change_mode = mode_shared.write().unwrap();
                            *change_mode = AppMode::GameMode;
                            break;
                        }
                        AppAction::VirtualKeyboard => {
                            let mut change_mode = mode_shared.write().unwrap();
                            *change_mode = AppMode::KeyboardMode;
                            break;
                        }
                        AppAction::Exec(exec_val) => {
                            processor::run_cmd(&exec_val);
                            button_control.combo_release();
                            thread::sleep(Duration::from_millis(20));
                        }
                        AppAction::PyExec(py_file) => {
                            processor::run_py(&py_file);
                            button_control.combo_release();
                            thread::sleep(Duration::from_millis(20));
                        }
                        AppAction::MacroKeys(macro_data) => {
                            key_pressed = processor::exec_macro(macro_data, &mut device);
                            button_control.combo_release();
                            thread::sleep(Duration::from_millis(20));
                        }
                        AppAction::ClipboardBuffer => {
                            if let Some(macro_data) = clipboard.as_ref() {
                                debug!(
                                    "{}",
                                    format!("Run macro clipboard: {:?}", macro_data).purple()
                                );
                                key_pressed = processor::exec_macro(&macro_data, &mut device);
                                button_control.combo_release();
                                thread::sleep(Duration::from_millis(20));
                            } else {
                                debug!("{}", format!("Copy to clipboard"));
                                processor::paste_clipboard(&mut device);
                                button_control.combo_release();
                                thread::sleep(Duration::from_millis(20));
                            }
                        }
                        _ => (),
                    }
                }
            } else {
                button_control.clear_data();
            }
        }

        thread::sleep(Duration::from_millis(10));
    }

    let new_mode = mode_shared.read().unwrap();

    return Some((*new_mode).clone());
}
