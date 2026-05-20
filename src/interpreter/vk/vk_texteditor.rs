use eframe::egui::{self, Context, Id, Ui};

pub struct VKEditor {
    cursor: usize, //Numero de carateres
    lock_keyboard: bool,
    pub text: String,
    pub text_edit_id: Id,
}

impl VKEditor {
    pub fn new() -> Self {
        Self {
            cursor: 0,
            lock_keyboard: false,
            text: "".to_string(),
            text_edit_id: Id::new("vk_text_editor"),
        }
    }

    fn set_cursor_index(&mut self, ctx: &Context, new_index: usize) {
        if let Some(mut state) = egui::TextEdit::load_state(ctx, self.text_edit_id) {
            let ccursor = egui::text::CCursor::new(new_index);
            let cursor_range = egui::text::CCursorRange::one(ccursor);

            state.cursor.set_char_range(Some(cursor_range));
            state.store(ctx, self.text_edit_id);

            // Garante que o cursor não passe do número total de caracteres
            let char_count = self.text.chars().count();
            self.cursor = new_index.min(char_count);
        }
    }

    // Função auxiliar para converter o índice de caractere em índice de byte seguro
    fn byte_index(&self, char_idx: usize) -> usize {
        self.text
            .char_indices()
            .nth(char_idx)
            .map(|(b_idx, _)| b_idx)
            .unwrap_or_else(|| self.text.len())
    }

    pub fn set_lock(&mut self, lock: bool) {
        self.lock_keyboard = lock;
        if lock {
            self.clear();
        }
    }

    pub fn add_str(&mut self, value: &str, ui: &mut Ui) {
        if !self.lock_keyboard {
            let b_idx = self.byte_index(self.cursor);
            self.text.insert_str(b_idx, value);

            // ".com" tem 4 caracteres
            self.set_cursor_index(ui.ctx(), self.cursor + 4);
        }
    }

    pub fn add_char(&mut self, c: char, ui: &mut Ui) {
        if !self.lock_keyboard {
            let b_idx = self.byte_index(self.cursor);
            self.text.insert(b_idx, c);
            self.set_cursor_index(ui.ctx(), self.cursor + 1);
        }
    }

    pub fn remove_char(&mut self, ui: &mut Ui) {
        if !self.lock_keyboard && self.cursor > 0 && !self.text.is_empty() {
            let b_idx = self.byte_index(self.cursor - 1);
            self.text.remove(b_idx);
            self.set_cursor_index(ui.ctx(), self.cursor - 1);
        }
    }

    pub fn move_left(&mut self, ui: &mut Ui) {
        if !self.lock_keyboard && self.cursor > 0 {
            self.set_cursor_index(ui.ctx(), self.cursor - 1);
        }
    }

    pub fn move_right(&mut self, ui: &mut Ui) {
        if !self.lock_keyboard && self.cursor < self.text.chars().count() {
            self.set_cursor_index(ui.ctx(), self.cursor + 1);
        }
    }

    pub fn clear(&mut self) {
        if !self.text.is_empty() {
            self.text.clear();
            self.cursor = 0;
        }
    }
}
