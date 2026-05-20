use eframe::egui::{self};
use egui::Color32;

#[derive(Clone, Copy, PartialEq)]
pub enum KeyboardLayout {
    GeneralKeys,
    OtherKeys,
    AccentKeys,
    CircunflexKeys,
    TildeKeys,
    CrasisKeys,
}

pub const BLUE_COLOR: Color32 = Color32::from_rgb(0, 120, 215);
pub const RED_COLOR: Color32 = Color32::from_rgb(255, 0, 0);
pub const YELLOW_COLOR: Color32 = Color32::from_rgb(255, 204, 0);
