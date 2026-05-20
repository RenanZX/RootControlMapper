use crate::controller::keyboard_controller::StatusButton;
use colored::*;
use evdev::Key;
use gilrs::{Axis, Button};

pub trait OrangeColor {
    fn orange(&self) -> ColoredString;
}

impl<T: AsRef<str>> OrangeColor for T {
    fn orange(&self) -> ColoredString {
        self.as_ref().truecolor(255, 165, 0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputController {
    Gamepad(Button),
    RecordBtn(StatusButton),
    InvalidBtn,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClickType {
    PressedClick,
    DoubleClick,
    LongPress,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AppMode {
    MouseMode,
    GameMode,
    KeyboardMode,
}

impl AppMode {
    pub fn get_file_name(&self) -> &str {
        macro_rules! local_path {
            ($file:expr) => {
                if cfg!(debug_assertions) {
                    concat!("environment/debug/json/", $file)
                } else {
                    concat!("json/", $file)
                }
            };
        }
        match self {
            Self::MouseMode => local_path!("/mouse_mode.json"),
            Self::GameMode => local_path!("/game_mode.json"),
            Self::KeyboardMode => "",
        }
    }
}

#[derive(Clone, Debug)]
pub enum AppAction {
    Exec(String),
    PyExec(String),
    ChangeMode,
    VirtualKeyboard,
    ClipboardBuffer,
    MoveAxis(AxisValue),
    MacroKeys(Vec<Key>),
}

impl AppAction {
    pub fn get_move_axis(data_map: &Vec<MapCmd>) -> Option<&AxisValue> {
        let move_ax = data_map.iter().find_map(|val_map| {
            if let Some(AppAction::MoveAxis(axis_val)) = &val_map.action {
                Some(axis_val) // Retorna o valor encontrado (pode precisar de .clone() dependendo do tipo)
            } else {
                None // Ignora os outros elementos
            }
        });
        return move_ax;
    }
    pub fn is_move_axis(map_cmd: &MapCmd) -> bool {
        matches!(&map_cmd.action, Some(AppAction::MoveAxis(_)))
    }
}

#[derive(Clone, Debug)]
pub struct MapCmd {
    //#[validate(custom = "is_valid_controller_sec")]
    pub botoes: Vec<InputController>,
    pub action: Option<AppAction>,
    pub click_type: ClickType,
}

// fn is_valid_controller_sec(botoes: &Vec<InputController>)->Result<(), ValidationError> {
//     for botao in botoes {
//         if InputController::Gamepad(botao)
//     }
// }

#[derive(Copy, Clone, Debug)]
pub struct AxisValue {
    pub x: Axis,
    pub y: Axis,
}

pub fn set_ls_axis() -> AxisValue {
    AxisValue {
        x: Axis::LeftStickX,
        y: Axis::LeftStickY,
    }
}

pub fn set_rs_axis() -> AxisValue {
    AxisValue {
        x: Axis::RightStickX,
        y: Axis::RightStickY,
    }
}
