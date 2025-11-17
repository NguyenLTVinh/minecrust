use crate::block_suggester::{BlockSuggester, BlockSuggestion};
use crate::text::TextRenderer;

pub struct CommandPrompt {
    pub input: String,
    pub blink_timer: f32,
    pub blink_interval: f32,
    pub show_cursor: bool,
    backspace_held_time: f32,
    backspace_hold_delay: f32,
    backspace_repeat_interval: f32,
    pub message: String,
    pub message_timer: f32,
    pub message_duration: f32,
    pub suggestions: Vec<BlockSuggestion>,
    pub selected_suggestion_index: usize,
    pub show_suggestions: bool,
}

impl CommandPrompt {
    pub fn new() -> Self {
        CommandPrompt {
            input: String::new(),
            blink_timer: 0.0,
            blink_interval: 0.5,
            show_cursor: true,
            backspace_held_time: 0.0,
            backspace_hold_delay: 0.3,
            backspace_repeat_interval: 0.05,
            message: String::new(),
            message_timer: 0.0,
            message_duration: 3.0,
            suggestions: Vec::new(),
            selected_suggestion_index: 0,
            show_suggestions: false,
        }
    }

    pub fn update_suggestions(&mut self) {
        if self.input.starts_with("use ") && !self.input.ends_with(";") {
            let input_part = self.input.strip_prefix("use ").unwrap_or("");
            self.suggestions = BlockSuggester::suggest(input_part, 5);
            self.show_suggestions = !self.suggestions.is_empty();
            self.selected_suggestion_index = 0;
        } else {
            self.show_suggestions = false;
            self.suggestions.clear();
        }
    }

    pub fn move_suggestion_down(&mut self) {
        if self.show_suggestions && !self.suggestions.is_empty() {
            self.selected_suggestion_index =
                (self.selected_suggestion_index + 1) % self.suggestions.len();
        }
    }

    pub fn move_suggestion_up(&mut self) {
        if self.show_suggestions && !self.suggestions.is_empty() {
            if self.selected_suggestion_index == 0 {
                self.selected_suggestion_index = self.suggestions.len() - 1;
            } else {
                self.selected_suggestion_index -= 1;
            }
        }
    }

    pub fn apply_suggestion(&mut self) {
        if self.show_suggestions && self.selected_suggestion_index < self.suggestions.len() {
            let suggestion = &self.suggestions[self.selected_suggestion_index];
            self.input = format!("use {};", suggestion.block_name);
        }
    }

    pub fn add_char(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn backspace(&mut self) {
        if !self.input.is_empty() {
            self.input.pop();
        }
    }

    pub fn clear(&mut self) {
        self.input.clear();
    }

    pub fn on_backspace_press(&mut self) {
        self.backspace();
        self.backspace_held_time = 0.0;
    }

    pub fn update_backspace_hold(&mut self, delta_time: f32) {
        self.backspace_held_time += delta_time;

        if self.backspace_held_time > self.backspace_hold_delay {
            let excess_time = self.backspace_held_time - self.backspace_hold_delay;
            if excess_time % self.backspace_repeat_interval < delta_time {
                self.backspace();
            }
        }
    }

    pub fn on_backspace_release(&mut self) {
        self.backspace_held_time = 0.0;
    }

    pub fn get_display_text(&self) -> String {
        let mut text = String::from("/");
        text.push_str(&self.input);
        if self.show_cursor {
            text.push('_');
        }
        text
    }

    pub fn update(&mut self, delta_time: f32) {
        self.blink_timer += delta_time;
        if self.blink_timer >= self.blink_interval {
            self.blink_timer -= self.blink_interval;
            self.show_cursor = !self.show_cursor;
        }
    }

    pub fn reset(&mut self) {
        self.input.clear();
        self.blink_timer = 0.0;
        self.show_cursor = true;
    }

    pub fn toggle(&mut self) {
        self.reset();
    }

    pub fn set_message(&mut self, msg: String) {
        self.message = msg;
        self.message_timer = 0.0;
    }

    pub fn render_message(
        &self,
        text_renderer: &TextRenderer,
        width: u32,
        height: u32,
        scale: f32,
        time: f32,
    ) {
        if self.message_timer < self.message_duration {
            text_renderer.render_text(&self.message, 10.0, 30.0, scale, width, height, time);
        }
    }

    pub fn render(
        &self,
        text_renderer: &TextRenderer,
        width: u32,
        height: u32,
        scale: f32,
        time: f32,
    ) {
        let prompt_text = self.get_display_text();
        text_renderer.render_text(&prompt_text, 10.0, 10.0, scale, width, height, time);

        if !self.input.starts_with("use ") || self.input.ends_with(";") {
            return;
        }

        if self.show_suggestions && !self.suggestions.is_empty() {
            let mut y_offset = 35.0;
            for (i, suggestion) in self.suggestions.iter().enumerate() {
                let indicator = if i == self.selected_suggestion_index {
                    "> "
                } else {
                    "  "
                };
                let line = format!("{}{}", indicator, suggestion.block_name);
                text_renderer.render_text(&line, 20.0, y_offset, scale, width, height, time);
                y_offset += 20.0;
            }
        }
    }
}
