use crate::conversor;
use crate::types::{AppAction, AppMode, ClickType, InputController, MapCmd};
use crate::types::{set_ls_axis, set_rs_axis};
use evdev::Key;
use log::debug;
use serde::Deserialize; // Leitura de JSON no rust
use std::fs; //Manipulacao de arquivos no rust
use validator::{Validate, ValidationError};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ClickTypeStr {
    PressedClick(bool),
    DoubleClick(bool),
    LongPress(bool),
    DefaultClick,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AppActionStr {
    Exec(String),
    PyExec(String),
    ChangeMode(bool),
    VirtualKeyboard(bool),
    ClipboardBuffer(bool),
    MouseMove(bool),
    MacroKeys(Vec<String>),
}

#[derive(Deserialize, Debug, Validate)]
pub struct MapCmdStr {
    buttons: Vec<String>,
    #[serde(flatten)]
    action: Option<AppActionStr>,
    #[serde(flatten)]
    click_type: Option<ClickTypeStr>,
}

pub fn contains_rec(data_map: &Vec<MapCmd>) -> bool {
    data_map.iter().any(|map_cmd| {
        map_cmd
            .botoes
            .iter()
            .any(|b| matches!(b, InputController::RecordBtn(_)))
    })
}

pub fn print_data(dados_json: &Vec<MapCmd>) {
    for (i, item) in (&dados_json).iter().enumerate() {
        debug!("Item {}", i);
        if !AppAction::is_move_axis(item) {
            debug!("Combo: {:?}", item.botoes);
        }
        if let Some(action) = &item.action {
            match action {
                AppAction::Exec(cmd) => debug!("Cmd: {:?}", cmd),
                AppAction::PyExec(pyfile) => debug!("PyScript: {:?}", pyfile),
                AppAction::MacroKeys(macro_keys) => debug!("Macros: {:?}", macro_keys),
                AppAction::MoveAxis(axis) => debug!("Move Axis: {:?}", axis),
                _ => (),
            }
        }
    }
}

fn validate_buttons(raw_buttons: &Vec<String>) -> Result<Vec<InputController>, ValidationError> {
    let botoes = raw_buttons
        .iter()
        .map(|botao_str| {
            let button_value = conversor::string_to_btn(botao_str);
            if button_value == InputController::InvalidBtn {
                let invalid = format!(
                    "JSON Read failed: '{}' is not a valid button. Review your defined buttons",
                    botao_str
                );
                return Err(ValidationError::new("json_read_failed").with_message(invalid.into()));
            }
            Ok(button_value)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(botoes)
}

fn validate_keys(raw_keys: &Vec<String>) -> Result<Vec<Key>, ValidationError> {
    let macro_keys: Vec<Key> = raw_keys
        .iter()
        .map(|key_name| {
            conversor::string_to_key(key_name).ok_or_else(|| {
                let invalid = format!(
                    "JSON Read failed: '{}' is not a valid key. Review your defined macro keys",
                    key_name
                );

                ValidationError::new("json_read_failed").with_message(invalid.into())
            })
        })
        .collect::<Result<Vec<Key>, ValidationError>>()?;
    Ok(macro_keys)
}

fn str_to_map(map_cmd: &MapCmdStr, app_mode: &AppMode) -> Result<MapCmd, ValidationError> {
    let mut buttons: Vec<InputController> = vec![];
    let mut click_type: ClickType = ClickType::PressedClick;
    let mut action_cmd: Option<AppAction> = None;

    if let Some(action) = &map_cmd.action {
        match action {
            AppActionStr::ChangeMode(change_mode) => {
                action_cmd = change_mode.then_some(AppAction::ChangeMode);
                buttons = validate_buttons(&map_cmd.buttons)?;
            }
            AppActionStr::Exec(exec_val) => {
                action_cmd = Some(AppAction::Exec(exec_val.to_string()));
                buttons = validate_buttons(&map_cmd.buttons)?;
            }
            AppActionStr::PyExec(py_file) => {
                action_cmd = Some(AppAction::PyExec(py_file.to_string()));
                buttons = validate_buttons(&map_cmd.buttons)?;
            }
            AppActionStr::VirtualKeyboard(virtual_keyboard) => {
                action_cmd = virtual_keyboard.then_some(AppAction::VirtualKeyboard);
                buttons = validate_buttons(&map_cmd.buttons)?;
            }
            AppActionStr::ClipboardBuffer(clipboard_buffer) => {
                action_cmd = clipboard_buffer.then_some(AppAction::ClipboardBuffer);
                buttons = validate_buttons(&map_cmd.buttons)?;
            }
            AppActionStr::MouseMove(mouse_move) => {
                if *mouse_move && *app_mode == AppMode::MouseMode {
                    let stick_rl: &str = &map_cmd.buttons[0];
                    if stick_rl == "RIGHT_STICK" {
                        action_cmd = Some(AppAction::MoveAxis(set_rs_axis()));
                    } else {
                        action_cmd = Some(AppAction::MoveAxis(set_ls_axis()));
                    }
                }
            }
            AppActionStr::MacroKeys(macro_keys) => {
                let macro_data: Vec<Key> = validate_keys(macro_keys)?;
                action_cmd = Some(AppAction::MacroKeys(macro_data));
                buttons = validate_buttons(&map_cmd.buttons)?;
            }
        }
    }

    if let Some(cl_type) = &map_cmd.click_type {
        match cl_type {
            ClickTypeStr::DoubleClick(double_click) => {
                if *double_click {
                    click_type = ClickType::DoubleClick
                };
            }
            ClickTypeStr::LongPress(long_press) => {
                if *long_press {
                    click_type = ClickType::LongPress
                };
            }
            ClickTypeStr::PressedClick(pressed_click) => {
                if *pressed_click {
                    click_type = ClickType::PressedClick
                };
            }
            _ => (),
        }
    }

    Ok(MapCmd {
        botoes: buttons,
        action: action_cmd,
        click_type,
    })
}

pub fn get_data(tipo_arq: AppMode) -> Result<Vec<MapCmd>, ValidationError> {
    let arquivo_json =
        fs::read_to_string(tipo_arq.get_file_name()).expect("Erro ao ler json, arquivo nao existe");
    let data_json: Vec<MapCmdStr> = serde_json::from_str(&arquivo_json)
        .expect("Erro ao ler JSON, verifique se a formatação está correta!");
    data_json
        .into_iter()
        .map(|map_cmd| {
            //Read each command defined of the JSON file
            let new_mapcmd = str_to_map(&map_cmd, &tipo_arq);
            new_mapcmd
        })
        .collect()
}
