use log::debug;

use crate::controller::keyboard_controller::StatusButton;
use crate::types::AppAction;
use crate::types::{ClickType, InputController, MapCmd};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const MAX_COMBO_LENGTH: usize = 2;
const COMBO_TIMEOUT: Duration = Duration::from_micros(150);
const LONG_TIMEIN: Duration = Duration::from_millis(30);
const SHORT_TIMEIN: Duration = Duration::from_millis(1);

pub struct ButtonCombo {
    button_pressed: VecDeque<InputController>,
    last_input_time: Instant,
}

impl ButtonCombo {
    pub fn create() -> Self {
        Self {
            button_pressed: VecDeque::with_capacity(MAX_COMBO_LENGTH),
            last_input_time: Instant::now(),
        }
    }

    fn check_record_long_press(&mut self, new_btn: &InputController) -> bool {
        *new_btn == InputController::RecordBtn(StatusButton::LongPress)
            && (self.button_pressed[0] == InputController::RecordBtn(StatusButton::Press)
                || self.button_pressed[0] == InputController::RecordBtn(StatusButton::LongPress))
    }

    fn is_record_long_press(&mut self, button: &InputController) -> bool {
        *button == InputController::RecordBtn(StatusButton::Press)
            && self.button_pressed[0] == InputController::RecordBtn(StatusButton::LongPress)
    }

    pub fn add_button(&mut self, btn: InputController) {
        self.last_input_time = Instant::now();
        if self.button_pressed.len() >= MAX_COMBO_LENGTH {
            self.button_pressed.pop_front();
        }
        if self.check_record_long_press(&btn) {
            self.button_pressed.pop_front();
        }
        self.button_pressed.push_back(btn);
        // println!("Buttons: {:?}", self.button_pressed);
    }
    /*
        pub fn pop_button(&mut self, btn: InputController) {
          if let Some(index) = self.button_pressed.iter().position(|&button| button == btn) {
              self.button_pressed.remove(index);
          }
        }

        pub fn print_buttons(&self) {
            println!("Botoes pressionados {:?}", self.button_pressed);
        }
    */
    /*
        pub fn get_current_seq(&self)->Vec<Button> {
            self.button_pressed.iter().cloned().collect()
        }
    */
    pub fn clear_data(&mut self) {
        if !self.button_pressed.is_empty() && self.last_input_time.elapsed() > COMBO_TIMEOUT {
            self.button_pressed.clear();
        }
    }

    pub fn combo_release(&mut self) {
        self.button_pressed.clear();
        self.last_input_time = Instant::now();
    }

    pub fn check_button_pressed<'a>(&mut self, data_map: &'a Vec<MapCmd>) -> Option<&'a MapCmd> {
        let duration = Instant::now() - self.last_input_time;

        for data in data_map {
            let botoes = &data.botoes;
            let click_type = &data.click_type;

            debug!(
                "Buttons added({:?}): {:?} \nButton Map({:?}): {:?} \nClick Type: {:?}",
                self.button_pressed.len(),
                self.button_pressed,
                botoes.len(),
                botoes,
                click_type
            );
            // Move axis e o mouse e nesse caso o tamanho do array e 0
            if AppAction::is_move_axis(data) {
                continue;
            }

            if botoes.len() > 1 {
                if botoes.len() == self.button_pressed.len() {
                    let match_all = botoes.iter().all(|b| self.button_pressed.contains(b));

                    if match_all && duration < SHORT_TIMEIN {
                        return Some(data);
                    }
                }
            } else if botoes[0] == self.button_pressed[0] || self.is_record_long_press(&botoes[0]) {
                match click_type {
                    ClickType::DoubleClick => {
                        if self.button_pressed.len() > 1
                            && self.button_pressed[0] == self.button_pressed[1]
                        {
                            return Some(data);
                        }
                    }
                    ClickType::LongPress => match self.button_pressed[0] {
                        InputController::Gamepad(_) => {
                            if duration >= LONG_TIMEIN {
                                return Some(data);
                            }
                        }
                        InputController::RecordBtn(btn) => {
                            if btn == StatusButton::LongPress {
                                return Some(data);
                            }
                        }
                        InputController::InvalidBtn => return None,
                    },
                    ClickType::PressedClick => match self.button_pressed[0] {
                        InputController::Gamepad(_) => {
                            return Some(data);
                        }
                        InputController::RecordBtn(btn) => {
                            if btn == StatusButton::Press {
                                return Some(data);
                            }
                        }
                        InputController::InvalidBtn => return None,
                    },
                }
            }
        }
        return None;
    }
}
