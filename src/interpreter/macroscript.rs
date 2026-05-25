use colored::*;
use gilrs::{Axis, Button, Event, EventType, Gilrs};
use log::{debug, info};
use std::thread;
use std::time::Duration;

use crate::controller::button_controller::ButtonCombo;
use crate::controller::keyboard_controller::StatusButton;
use crate::interpreter::gamepad_utils::*;
use crate::processor;
use crate::types::InputController::{Gamepad, RecordBtn};

use crate::types::{AppAction, AppMode, MapCmd};

use evdev::uinput::VirtualDevice;

pub fn read_input_controller(data_map: Vec<MapCmd>) -> Option<AppMode> {
    let mut gilrs = Gilrs::new().unwrap();

    // 1. Criar o teclado virtual
    let mut device: VirtualDevice = build_kb_device(&data_map);

    // Pausa crucial para o sistema operacional registrar o novo "hardware"
    thread::sleep(Duration::from_millis(300));

    let (mut player1_id, mut xbox_keyboard_option) = check_gamepad(&mut gilrs, &data_map);
    let mut button_control = ButtonCombo::create();

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
                        botao_pressionado = true
                    }
                    EventType::AxisChanged(axis, value, _) => {
                        // Analógicos e Gatilhos (valor de -1.0 a 1.0 ou 0.0 a 1.0)
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
            thread::sleep(Duration::from_millis(300));
            button_control = ButtonCombo::create();
            gilrs = Gilrs::new().unwrap();
            let (new_id, new_kbd) = check_gamepad(&mut gilrs, &data_map);
            player1_id = new_id;
            xbox_keyboard_option = new_kbd;
            continue;
        }

        if !botao_pressionado {
            if let Some(kbd_arc) = xbox_keyboard_option.as_mut() {
                //Try Lock e usado para nao impedir a funcao de correr
                if let Ok(xbox_keyboard) = kbd_arc.try_lock() {
                    let record_btn = xbox_keyboard.get_record_button();
                    if record_btn != StatusButton::Realese {
                        button_control.add_button(RecordBtn(record_btn));
                        botao_pressionado = true;
                    }
                }

                /*
                match record_btn {
                   StatusButton::Press => println!("Record Pressionado!"),
                   StatusButton::LongPress => println!("Record Pressionado longo!"),
                   StatusButton::Realese => println!("Record Solto!"),
                }*/
            }
        }

        if botao_pressionado {
            // button_control.print_buttons();
            let value_chacked: Option<&MapCmd> = button_control.check_button_pressed(&data_map);
            if let Some(data_checked) = value_chacked {
                // println!("Value checked: {:?}", data_checked);
                if let Some(action) = &data_checked.action {
                    match action {
                        AppAction::ChangeMode => {
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
                            processor::exec_macro(&macro_data, &mut device);
                            button_control.combo_release();
                            thread::sleep(Duration::from_millis(20));
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

    return Some(AppMode::MouseMode);
}
