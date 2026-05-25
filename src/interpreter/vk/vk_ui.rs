use super::vk_clipboard::VKClipboard;
use super::vk_kb_layout::{get_keyboard_layout, is_macro_key};
use super::vk_texteditor::VKEditor;
use super::vk_types::KeyboardLayout;
use super::vk_types::KeyboardLayout::{
    AccentKeys, CircunflexKeys, CrasisKeys, GeneralKeys, OtherKeys, TildeKeys,
};
use super::vk_types::{BLUE_COLOR, RED_COLOR, YELLOW_COLOR};
use eframe::egui::{self};
use egui::{Color32, CornerRadius, RichText, Sense, vec2};
use evdev::Key;
use gilrs::{Button, Event, EventType, Gilrs};
use std::sync::mpsc::{Sender, channel};

use crate::helper::open_helper;
use crate::version::get_version;
use crate::{
    interpreter::gamepad_utils::get_virtual_player,
    types::{AppMode, InputController},
};
use colored::*;

// Struct do teclado virtual
struct KeyboardApp {
    vktext: VKEditor,       //Editor de texto
    shift: bool,            //boolean do Shift
    caps: bool,             //bolean do capslock
    clipboard: VKClipboard, //Clipboard do teclado virtual
    gilrs: Gilrs,           //lib que recebe os dados do joystick
    selected_row: usize,    // Seleção das teclas
    selected_col: usize,
    trigger_click: bool,     // Trigger para a ação de clicar ou não em uma tecla
    pos: egui::Pos2,         // Posição da janela
    stick_delta: egui::Vec2, //Delta do analogico para atualizar a posição da janela
    dir_pressed: InputController, // Direção pressionada do controle
    last_move_time: f64,     // Tempo da última repetição do direcional dpad
    move_delay_active: bool, // verificação para identificar quando o direcional esta sendo
    // pressionado
    kb_layout: KeyboardLayout,       //Layout do teclado
    tx: Option<Sender<VKClipboard>>, //Canal para o envio de dados do clipboard
}

impl Default for KeyboardApp {
    fn default() -> Self {
        Self {
            vktext: VKEditor::new(),
            shift: false,
            caps: false,
            clipboard: VKClipboard::new(),
            gilrs: Gilrs::new().unwrap(),
            selected_row: 0,
            selected_col: 0,
            trigger_click: false,
            pos: egui::pos2(100.0, 100.0),
            stick_delta: egui::vec2(0.0, 0.0),
            dir_pressed: InputController::InvalidBtn,
            last_move_time: 0.0,
            move_delay_active: false,
            kb_layout: GeneralKeys,
            tx: None,
        }
    }
}

fn setup_font(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let name_font = "noto-sans";

    fonts.font_data.insert(
        name_font.to_string(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "assets/noto-sans.ttf" // Caminho do seu arquivo
        ))),
    );

    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, name_font.to_string());

    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, name_font.to_string());

    ctx.set_fonts(fonts);
}

impl eframe::App for KeyboardApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.set_visuals(egui::Visuals::dark());
        ui.global_style_mut(|style| {
            style.spacing.item_spacing = vec2(4.0, 4.0);

            // Bordas arredondadas
            let rounding = CornerRadius::same(8);
            style.visuals.menu_corner_radius = rounding;
            style.visuals.window_corner_radius = rounding;
            // Cores escuras estilo Windows/Mobile
            style.visuals.window_fill = Color32::from_gray(15);
            style.visuals.panel_fill = Color32::from_gray(15);
            style.visuals.widgets.inactive.bg_fill = Color32::from_gray(40);
        });
        setup_font(ui.ctx());
        // 2. Definição das linhas com PESOS (Weights)
        let mut rows: Vec<Vec<(&str, f32)>> = get_keyboard_layout(self.kb_layout);
        self.check_controller_input(&mut rows, ui);
        self.update_dir_pressed(&rows, ui);
        self.update_screen_position(ui);

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let label_font_size = 14.0;
            // Remover qualquer margem/padding interna do painel
            ui.spacing_mut().item_spacing = vec2(4.0, 4.0);

            // Campo de texto
            ui.add(
                egui::TextEdit::multiline(&mut self.vktext.text)
                    .id(self.vktext.text_edit_id)
                    .desired_width(f32::INFINITY)
                    .desired_rows(2)
                    .hint_text("Write buffer here..."),
            );
            ui.separator();

            let total_height = ui.available_height() - label_font_size;
            let row_height = total_height / rows.len() as f32;

            for (r_idx, row) in rows.iter().enumerate() {
                ui.horizontal(|ui| {
                    let total_weight: f32 = row.iter().map(|(_, w)| w).sum();
                    let total_width = ui.available_width();

                    // Cálculo para preencher a largura exata, descontando os espaçamentos
                    let num_gaps = (row.len() - 1) as f32;
                    let spacing = 4.0;
                    let total_spacing_width = num_gaps * spacing;
                    let usable_width = total_width - total_spacing_width;

                    for (c_idx, &(key, weight)) in row.iter().enumerate() {
                        let mut key_value: String = key.to_string();
                        let width = (usable_width / total_weight) * weight;
                        let btn_size = vec2(width, row_height - 5.0);
                        let is_selected = r_idx == self.selected_row && c_idx == self.selected_col;

                        if (self.shift || self.caps) && key_value.len() == 1 {
                            if let Some(c) = key_value.chars().next()
                                && c.is_alphabetic()
                            {
                                key_value = key_value.to_uppercase();
                            }
                        }
                        // --- ESTILIZAÇÃO XBOX ---
                        let mut text_color = Color32::WHITE;
                        let mut button_color = Color32::from_gray(45);

                        match key {
                            "Enter" => {
                                if self.kb_layout == GeneralKeys {
                                    key_value = "Copy ☰".to_string();
                                } else {
                                    key_value = "Enter ☰".to_string();
                                }
                            }
                            "Right" => key_value = "> RB".to_string(),
                            "Left" => key_value = "< LB".to_string(),
                            "Space" => {
                                // Espaço com ícone Y (Amarelo)
                                text_color = YELLOW_COLOR; // Amarelo Xbox
                                key_value = "Space (Y)".to_string(); // Ou use um ícone unicode se preferir
                            }
                            "Backspace" => {
                                // Backspace com ícone X (Azul) - assumindo que você tenha Backspace no layout
                                if self.kb_layout == GeneralKeys {
                                    key_value = "⬅ (X)".to_string();
                                } else {
                                    key_value = "Clear (X)".to_string();
                                }
                                text_color = BLUE_COLOR;
                            }
                            "Close" => {
                                key_value = "Close (B)".to_string();
                                text_color = RED_COLOR;
                            }
                            key if is_macro_key(key) => {
                                if self.clipboard.has_key(key) {
                                    button_color = BLUE_COLOR;
                                }
                            }
                            "Shift" => {
                                if self.shift {
                                    button_color = BLUE_COLOR;
                                }
                                key_value = "Shift (LS)".to_string()
                            }
                            "Helper" => key_value = "Helper (RS)".to_string(),
                            "Change" => {
                                if self.kb_layout == GeneralKeys {
                                    key_value = "#+= LT".to_string();
                                } else {
                                    key_value = "<Back LT".to_string();
                                }
                            }
                            "Change2" => key_value = "´~^`ç RT".to_string(),
                            "CapsLock" => {
                                if self.caps {
                                    button_color = Color32::WHITE;
                                    text_color = Color32::BLACK;
                                }
                                key_value = "Caps".to_string()
                            }
                            _ => {
                                if let Some(_c) = key.chars().next() {
                                    let k_value = format!("KEY_{}", key);
                                    if self.clipboard.has_key(&k_value) {
                                        button_color = BLUE_COLOR;
                                    }
                                }
                            }
                        }

                        let label = RichText::new(key_value).size(24.0).color(text_color);

                        let btn = ui.add(
                            egui::Button::new(label)
                                .min_size(btn_size)
                                .corner_radius(10)
                                .sense(Sense::click())
                                .fill(button_color),
                        );

                        if is_selected {
                            btn.request_focus();
                        }

                        if btn.clicked() || (is_selected && self.trigger_click) {
                            self.process_key(ui, key);
                        }
                    }
                });
            }
            let texto = egui::RichText::new("Move screen with (RS) Analog").size(label_font_size);
            let label = egui::Label::new(texto).wrap().halign(egui::Align::Center);
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                let m_width = ui.available_width();
                ui.add_sized([m_width, 0.0], label);
            });
        });
        self.trigger_click = false;

        // CORREÇÃO 2: Força o egui a rodar continuamente a 60fps para ouvir o Gilrs
        ui.request_repaint();
    }
}

impl KeyboardApp {
    // Cria o App com um canal para enviar dados do clipboard
    fn new(_cc: &eframe::CreationContext<'_>, tx: Sender<VKClipboard>) -> Self {
        Self {
            tx: Some(tx),
            ..Default::default()
        }
    }

    // Verifica a entrada do controle no teclado virtual
    fn check_controller_input(
        &mut self,
        layout: &mut Vec<Vec<(&'static str, f32)>>,
        ui: &mut egui::Ui,
    ) {
        let player1_id = get_virtual_player(&mut self.gilrs);
        let mut lt_pressed = false;
        while let Some(Event { id, event, .. }) = self.gilrs.next_event() {
            if Some(id) == player1_id {
                match event {
                    EventType::ButtonPressed(Button::DPadUp, _) => {
                        self.set_move(ui, Button::DPadUp);
                        self.move_dir(layout);
                    }
                    EventType::ButtonPressed(Button::DPadDown, _) => {
                        self.set_move(ui, Button::DPadDown);
                        self.move_dir(layout);
                    }
                    EventType::ButtonPressed(Button::DPadLeft, _) => {
                        self.set_move(ui, Button::DPadLeft);
                        self.move_dir(layout);
                    }
                    EventType::ButtonPressed(Button::DPadRight, _) => {
                        self.set_move(ui, Button::DPadRight);
                        self.move_dir(layout);
                    }
                    EventType::ButtonReleased(button, _) => {
                        if InputController::Gamepad(button) == self.dir_pressed {
                            self.dir_pressed = InputController::InvalidBtn;
                        }
                    }
                    EventType::ButtonPressed(Button::East, _) => {
                        // Botao B no Xbox
                        self.process_key(ui, "Close");
                    }
                    EventType::ButtonPressed(Button::South, _) => {
                        // Botão A no Xbox
                        self.trigger_click = true;
                    }
                    EventType::ButtonPressed(Button::North, _) => {
                        // Botão X
                        self.process_key(ui, "Backspace"); // Não precisa passar `ui` se refatorar
                    }
                    EventType::ButtonPressed(Button::RightTrigger, _) => {
                        self.process_key(ui, "Right");
                    }
                    EventType::ButtonPressed(Button::LeftTrigger, _) => {
                        self.process_key(ui, "Left");
                    }
                    EventType::ButtonPressed(Button::West, _) => {
                        // Botão Y
                        self.process_key(ui, "Space");
                    }
                    EventType::ButtonPressed(Button::Start, _) => {
                        self.process_key(ui, "Enter");
                    }
                    EventType::ButtonPressed(Button::LeftThumb, _) => {
                        // Left Tumb
                        self.process_key(ui, "Shift");
                    }
                    EventType::ButtonPressed(Button::RightThumb, _) => {
                        // Right Tumb
                        self.process_key(ui, "CapsLock");
                    }
                    EventType::AxisChanged(axis, value, _) => {
                        let deadzone = 0.1;
                        let val = if value.abs() > deadzone { value } else { 0.0 };

                        match axis {
                            gilrs::Axis::RightStickX => {
                                self.stick_delta.x = val;
                            }
                            gilrs::Axis::RightStickY => {
                                self.stick_delta.y = val;
                            }
                            gilrs::Axis::LeftZ => {
                                let ax = val.abs();
                                if ax <= 0.5 && !lt_pressed {
                                    self.process_key(ui, "Change");
                                    lt_pressed = true;
                                } else if ax >= 0.9 && lt_pressed {
                                    lt_pressed = false;
                                }
                            }
                            gilrs::Axis::RightZ => {
                                let ax = val.abs();
                                if ax <= 0.5 && !lt_pressed {
                                    self.process_key(ui, "Change2");
                                    lt_pressed = true;
                                } else if ax >= 0.9 && lt_pressed {
                                    lt_pressed = false;
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }

        self.selected_col = self.selected_col.min(layout[self.selected_row].len() - 1);
    }

    // Atualiza a posição da janela ao mover com (RS)
    fn update_screen_position(&mut self, ui: &mut egui::Ui) {
        let speed = 8.0; // Ajuste a sensibilidade aqui
        if self.stick_delta.length() > 0.0 {
            self.pos.x += self.stick_delta.x * speed;
            self.pos.y -= self.stick_delta.y * speed; // Y é invertido no egui

            // Envia o comando de reposicionamento
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::OuterPosition(self.pos));
            ui.request_repaint();
        }
    }

    //Atualiza a posição da seleção do direcional ao pressionar uma diração do dpad
    fn update_dir_pressed(&mut self, layout: &[Vec<(&'static str, f32)>], ui: &mut egui::Ui) {
        let current_time = ui.input(|i| i.time);
        let delay = 0.4; // 400ms antes de começar a repetir
        let interval = 0.1; // 100ms entre cada salto após o delay

        if self.dir_pressed != InputController::InvalidBtn {
            let elapsed = current_time - self.last_move_time;

            let should_move = if self.move_delay_active {
                elapsed >= delay
            } else {
                elapsed >= interval
            };

            if should_move {
                self.move_dir(&layout);
                self.last_move_time = current_time;
                self.move_delay_active = false; // Após o primeiro delay, vira repetição rápida
            }
        }
    }

    //Define a direção que o seletor do teclado irá seguir
    fn set_move(&mut self, ui: &mut egui::Ui, button: Button) {
        self.dir_pressed = InputController::Gamepad(button);
        self.last_move_time = ui.input(|i| i.time);
        self.move_delay_active = true;
    }

    // Move para a direção específica pressionada
    fn move_dir(&mut self, layout: &[Vec<(&'static str, f32)>]) {
        match self.dir_pressed {
            InputController::Gamepad(Button::DPadUp) => {
                self.selected_row = self.selected_row.saturating_sub(1);
            }
            InputController::Gamepad(Button::DPadDown) => {
                self.selected_row = (self.selected_row + 1).min(layout.len() - 1);
            }
            InputController::Gamepad(Button::DPadLeft) => {
                self.selected_col = self.selected_col.saturating_sub(1);
            }
            InputController::Gamepad(Button::DPadRight) => {
                self.selected_col =
                    (self.selected_col + 1).min(layout[self.selected_row].len() - 1);
            }
            _ => {}
        }
        // Garante que a coluna não fique fora de alcance ao mudar de linha
        self.selected_col = self.selected_col.min(layout[self.selected_row].len() - 1);
    }

    //Envia os dados do canal(macro definida ou texto copiado) para o modo mouse
    fn send_data(&mut self) {
        if self.vktext.text.len() > 0 {
            self.clipboard.copy_to_clipboard(self.vktext.text.clone());
        } else {
            if let Some(sender) = &self.tx {
                let _ = sender.send(self.clipboard.clone());
            }
        }
    }

    // Executa as ações da tecla e atualiza o campo de texto do teclado
    fn process_key(&mut self, ui: &mut egui::Ui, key: &str) {
        match key {
            "Shift" => {
                self.shift = !self.shift;
                self.clipboard.add_cmd(key);
            }
            "Right" => self.vktext.move_right(ui),
            "Left" => self.vktext.move_left(ui),
            "CapsLock" => self.caps = !self.caps,
            "Backspace" => match self.kb_layout {
                GeneralKeys | OtherKeys => {
                    self.vktext.remove_char(ui);
                }
                _ => {
                    self.vktext.clear();
                }
            },
            "Space" => self.vktext.add_char(' ', ui),
            "Enter" => {
                if self.kb_layout == GeneralKeys {
                    self.send_data();
                    self.vktext.clear();
                    ui.send_viewport_cmd(egui::ViewportCommand::Close);
                } else {
                    self.vktext.add_char('\n', ui);
                }
            }
            "www." => self.vktext.add_str("www.", ui),
            ".com" => self.vktext.add_str(".com", ui),
            key if is_macro_key(key) => self.clipboard.add_cmd(key),
            "Helper" => open_helper(),
            "Change" => {
                if self.kb_layout == GeneralKeys {
                    self.kb_layout = OtherKeys;
                } else {
                    self.kb_layout = GeneralKeys;
                }
            }
            "Change2" => {
                self.kb_layout = match self.kb_layout {
                    GeneralKeys => AccentKeys,
                    AccentKeys => CircunflexKeys,
                    CircunflexKeys => TildeKeys,
                    TildeKeys => CrasisKeys,
                    CrasisKeys => AccentKeys,
                    _ => GeneralKeys,
                };
            }
            "Close" => {
                ui.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            _ => {
                if let Some(mut c) = key.chars().next() {
                    if self.clipboard.is_macro_active() {
                        // Macro esta ativada, logo adiciona a tecla a macro
                        self.clipboard.add_cmd(&format!("KEY_{}", key));
                    } else {
                        if (self.shift || self.caps) && c.is_ascii_alphabetic() {
                            c = c.to_ascii_uppercase();
                        } else {
                            c = c.to_ascii_lowercase();
                        }
                        self.vktext.add_char(c, ui);
                        if self.shift && key.len() == 1 {
                            self.shift = false;
                        }
                    }
                }
            }
        }
        let macro_active = self.clipboard.is_macro_active();
        self.vktext.set_lock(macro_active);
    }
}

// Renderiza o teclado
pub fn render_vk() -> (AppMode, Option<Vec<Key>>) {
    #[cfg(target_os = "linux")]
    {
        // SAFETY: Modificamos a variável de ambiente logo no início do main,
        // antes de qualquer thread paralela ser gerada pelo eframe.
        unsafe {
            // 1. Diz ao winit para usar a API do X11
            std::env::set_var("WINIT_UNIX_BACKEND", "x11");

            // 2. Apaga o link direto com o display do Wayland
            // Isso força o app a recuar (fallback) para o XWayland/X11
            std::env::set_var("WAYLAND_DISPLAY", "");
        }
    }

    let (tx, rx) = channel::<VKClipboard>();
    let size_vk = egui::vec2(1100.0, 380.0); // x largura, y altura vec(x,y) 1100 largura,380 de
    // altura

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(size_vk)
            .with_min_inner_size(size_vk)
            .with_max_inner_size(size_vk)
            .with_resizable(false)
            .with_position(egui::pos2(268.0, 500.0))
            .with_always_on_top(),
        ..Default::default()
    };

    let app_title = format!("Virtual Keyboard Root Control Mapper v{}", get_version());

    let result = eframe::run_native(
        &app_title,
        options,
        Box::new(|cc| Ok(Box::new(KeyboardApp::new(cc, tx)))),
    );
    match result {
        Ok(_) => {
            if let Ok(mut clipboard) = rx.recv() {
                clipboard.print_keys();
                println!("{}", "Keyboard is sucessfully closed!".green());
                let data_cmd = clipboard.get_keys();

                return (AppMode::MouseMode, Some(data_cmd));
            }
            println!("{}", "Keyboard is sucessfully closed!".green());
            (AppMode::MouseMode, None)
        }
        Err(e) => {
            let error_keyboard = format!("Keyboard error: {}", e);
            eprintln!("{}", error_keyboard.red().bold());
            (AppMode::KeyboardMode, None)
        }
    }
}
