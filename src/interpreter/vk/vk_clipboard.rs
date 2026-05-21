use arboard::Clipboard;
use colored::*;
use evdev::Key;
use indexmap::IndexSet;

use crate::{
    conversor::string_to_key,
    interpreter::vk::vk_kb_layout::{is_function_key, is_macro_key},
};

#[derive(Clone, Debug)]
pub struct VKClipboard {
    cmds_pressed: IndexSet<String>,
    count_mkeys: i32,
}

fn get_key(key: &str) -> String {
    if is_function_key(key) {
        return format!("KEY_{}", key);
    } else {
        return key.to_string();
    }
}

impl VKClipboard {
    pub fn new() -> Self {
        Self {
            cmds_pressed: IndexSet::new(),
            count_mkeys: 0,
        }
    }

    fn add_count(&mut self) {
        self.count_mkeys += 1;
    }

    fn sub_count(&mut self) {
        if self.count_mkeys > 0 {
            self.count_mkeys -= 1;
        }
    }

    pub fn add_cmd(&mut self, key: &str) {
        if !self.cmds_pressed.contains(key) {
            match key {
                key if is_macro_key(key) => self.add_count(),
                _ => (),
            }
            let key_value = get_key(key);
            self.cmds_pressed.insert(key_value);
        } else {
            match key {
                key if is_macro_key(key) => self.sub_count(),
                _ => (),
            }
            let key_value = get_key(key);
            self.cmds_pressed.shift_remove(&key_value);
            if self.count_mkeys == 0 {
                self.clear();
            }
        }
    }

    fn clear(&mut self) {
        self.cmds_pressed.clear();
    }

    pub fn has_key(&mut self, key: &str) -> bool {
        let key_value = get_key(key);
        self.cmds_pressed.contains(&key_value)
    }

    pub fn is_macro_active(&mut self) -> bool {
        self.cmds_pressed.len() > 0
    }

    pub fn print_keys(&mut self) {
        for cmd in &self.cmds_pressed {
            println!("Cmd: {}", cmd);
        }
    }

    pub fn get_keys(&mut self) -> Vec<Key> {
        let cmd_keys: Vec<Key> = self
            .cmds_pressed
            .clone()
            .into_iter()
            .filter_map(|cmd| string_to_key(&cmd.to_uppercase()))
            .collect();
        cmd_keys
    }

    pub fn copy_to_clipboard(&self, value: String) {
        let mut clipboard = Clipboard::new().unwrap();

        match clipboard.set_text(value) {
            Ok(_) => println!("{}", "Texto copiado com sucesso!".green()),
            Err(e) => println!("{}", format!("Erro ao copiar: {}", e).red()),
        }
    }
}
