use evdev::Key as EvdevKey;
use gilrs::Button;

use crate::types::InputController::{Gamepad, RecordBtn};
use crate::{controller::keyboard_controller::StatusButton, types::InputController};

// Tabela de mapeamento de caracteres para teclas do Kernel Linux
pub fn char_to_key(c: &char) -> Option<EvdevKey> {
    match c.to_ascii_uppercase() {
        'A' => Some(EvdevKey::KEY_A),
        'B' => Some(EvdevKey::KEY_B),
        'C' => Some(EvdevKey::KEY_C),
        'D' => Some(EvdevKey::KEY_D),
        'E' => Some(EvdevKey::KEY_E),
        'F' => Some(EvdevKey::KEY_F),
        'G' => Some(EvdevKey::KEY_G),
        'H' => Some(EvdevKey::KEY_H),
        'I' => Some(EvdevKey::KEY_I),
        'J' => Some(EvdevKey::KEY_J),
        'K' => Some(EvdevKey::KEY_K),
        'L' => Some(EvdevKey::KEY_L),
        'M' => Some(EvdevKey::KEY_M),
        'N' => Some(EvdevKey::KEY_N),
        'O' => Some(EvdevKey::KEY_O),
        'P' => Some(EvdevKey::KEY_P),
        'Q' => Some(EvdevKey::KEY_Q),
        'R' => Some(EvdevKey::KEY_R),
        'S' => Some(EvdevKey::KEY_S),
        'T' => Some(EvdevKey::KEY_T),
        'U' => Some(EvdevKey::KEY_U),
        'V' => Some(EvdevKey::KEY_V),
        'W' => Some(EvdevKey::KEY_W),
        'X' => Some(EvdevKey::KEY_X),
        'Y' => Some(EvdevKey::KEY_Y),
        'Z' => Some(EvdevKey::KEY_Z),
        '0' => Some(EvdevKey::KEY_0),
        '1' => Some(EvdevKey::KEY_1),
        '2' => Some(EvdevKey::KEY_2),
        '3' => Some(EvdevKey::KEY_3),
        '4' => Some(EvdevKey::KEY_4),
        '5' => Some(EvdevKey::KEY_5),
        '6' => Some(EvdevKey::KEY_6),
        '7' => Some(EvdevKey::KEY_7),
        '8' => Some(EvdevKey::KEY_8),
        '9' => Some(EvdevKey::KEY_9),

        _ => None,
    }
}

fn function_key(f_key: &str) -> Option<EvdevKey> {
    let number_str = &f_key.strip_prefix("KEY_F")?;
    let number: i32 = number_str.parse().ok()?;
    match number {
        1 => Some(EvdevKey::KEY_F1),
        2 => Some(EvdevKey::KEY_F2),
        3 => Some(EvdevKey::KEY_F3),
        4 => Some(EvdevKey::KEY_F4),
        5 => Some(EvdevKey::KEY_F5),
        6 => Some(EvdevKey::KEY_F6),
        7 => Some(EvdevKey::KEY_F7),
        8 => Some(EvdevKey::KEY_F8),
        9 => Some(EvdevKey::KEY_F9),
        10 => Some(EvdevKey::KEY_F10),
        11 => Some(EvdevKey::KEY_F11),
        12 => Some(EvdevKey::KEY_F12),
        _ => None,
    }
}

// Mapeia strings de nomes de teclas (vindas do seu JSON) para EvdevKey
pub fn string_to_key(name: &str) -> Option<EvdevKey> {
    match name.to_uppercase().as_str() {
        "ALT" | "KEY_LEFTALT" | "KEY_RIGHTALT" => Some(EvdevKey::KEY_LEFTALT),
        "CTRL" | "KEY_LEFTCTRL" | "KEY_RIGHTCTRL" => Some(EvdevKey::KEY_LEFTCTRL),
        "SHIFT" | "KEY_LEFTSHIFT" | "KEY_RIGHTSHIFT" => Some(EvdevKey::KEY_LEFTSHIFT),
        "ARROW_RIGHT" => Some(EvdevKey::KEY_RIGHT),
        "ARROW_LEFT" => Some(EvdevKey::KEY_LEFT),
        "ARROW_UP" => Some(EvdevKey::KEY_UP),
        "ARROW_DOWN" => Some(EvdevKey::KEY_DOWN),
        "SCROLL_DOWN" => Some(EvdevKey::KEY_SCROLLDOWN),
        "SCROLL_UP" => Some(EvdevKey::KEY_SCROLLUP),
        "BACKSPACE" => Some(EvdevKey::KEY_BACKSPACE),
        "ENTER" | "RETURN" => Some(EvdevKey::KEY_ENTER),
        "ESC" | "ESCAPE" => Some(EvdevKey::KEY_ESC),
        "SPACE" => Some(EvdevKey::KEY_SPACE),
        "MOUSE_RIGHTCLICK" => Some(EvdevKey::BTN_RIGHT),
        "MOUSE_LEFTCLICK" => Some(EvdevKey::BTN_LEFT),
        // Se for uma tecla tipo "KEY_T", extrai o caractere 'T'
        n if n.starts_with("KEY_") && n.len() == 5 => char_to_key(&n.chars().nth(4).unwrap()),
        n if n.starts_with("KEY_F") && n.len() <= 7 => function_key(&n),
        _ => None,
    }
}

pub fn string_to_btn(button_name: &str) -> InputController {
    match button_name.to_uppercase().as_str() {
        "BTN_SELECT" => Gamepad(Button::Select),
        "BTN_START" => Gamepad(Button::Start),
        "BTN_HOME" | "BTN_MODE" => Gamepad(Button::Mode),
        "BTN_RECORD" => RecordBtn(StatusButton::Press),
        "BTN_LB" => Gamepad(Button::LeftTrigger),
        "BTN_RB" => Gamepad(Button::RightTrigger),
        "BTN_LT" => Gamepad(Button::LeftTrigger2),
        "BTN_RT" => Gamepad(Button::RightTrigger2),
        "BTN_LTHUMB" => Gamepad(Button::LeftThumb),
        "BTN_RTHUMB" => Gamepad(Button::RightThumb),
        "BTN_A" => Gamepad(Button::South),
        "BTN_B" => Gamepad(Button::East),
        "BTN_X" => Gamepad(Button::North),
        "BTN_Y" => Gamepad(Button::West),
        "DPAD_LEFT" => Gamepad(Button::DPadLeft),
        "DPAD_UP" => Gamepad(Button::DPadUp),
        "DPAD_DOWN" => Gamepad(Button::DPadDown),
        "DPAD_RIGHT" => Gamepad(Button::DPadRight),
        _ => InputController::InvalidBtn,
    }
}
