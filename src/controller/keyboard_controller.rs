use std::time::{Duration, Instant};
use evdev::{Device,InputEventKind, Key};

//const COMBO_TIMEOUT: Duration = Duration::from_millis(500);
const KEYBOARD_XBOX: &str = "2.4G XBOX 360 For Windows Keyboard";
const LONG_TIMEIN: Duration = Duration::from_millis(30);

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum StatusButton {
    Realese,
    Press,
    DoubleClick,
    LongPress,
}

pub struct KeyboardXbxController {
    device: Device,
    record_button: StatusButton,
    counter: i32,
    last_input_time: Instant
}

pub fn check_device()->bool {
    let mut devices = evdev::enumerate();
    let option = devices.find(|(_, dev)| {
        dev.name() == Some(&KEYBOARD_XBOX)
    });
    if let Some(_) = option {
        return true;
    }
    return false;
}

impl KeyboardXbxController {
    pub fn create()->Result<Self, String> {
        let mut devices = evdev::enumerate();
        let option = devices.find(|(_, dev)| {
            dev.name() == Some(&KEYBOARD_XBOX)
        });
        if let Some((_path, device)) = option {
            Ok(Self { 
                device: device,
                counter: 0,
                record_button: StatusButton::Realese,
                last_input_time: Instant::now() 
            })
        } else {
            Err(format!("Dispositivo não possui suporte a gravacao!"))
        } 
    }

    pub fn update_input(&mut self) {
        if let Ok(events) = self.device.fetch_events() {
            for event in events {
                //println!("events monitor {:?}", event);
                if let InputEventKind::Key(key) = event.kind() {

                    match key {
                        Key::KEY_SYSRQ => {
                            self.record_button = match event.value() {
                                0 => StatusButton::Realese,
                                1 => {
                                    if self.counter == 2 {
                                        self.counter = 0;
                                        StatusButton::DoubleClick
                                    } else {
                                        self.counter+=1;
                                        StatusButton::Press
                                    }
                                },
                                2 => {
                                    let duration = Instant::now() - self.last_input_time;
                                    if duration < LONG_TIMEIN {
                                        StatusButton::Press
                                    } else {
                                        StatusButton::LongPress
                                    }
                                },
                                _ => self.record_button,
                            };
                        }
                        _ => {}
                    }

                }
                self.last_input_time = Instant::now();
            }
        }
    }

    pub fn get_record_button(&self) -> StatusButton {
        self.record_button
    }
}
