use crate::text::TextRenderer;

pub struct CommandPrompt {
    pub input: String,
    pub blink_timer: f32,
    pub blink_interval: f32,
    pub show_cursor: bool,
    last_backspace_press_time: f32,
    backspace_press_count: u32,
    double_press_window: f32,
    backspace_held_time: f32,
    backspace_hold_delay: f32,
    backspace_repeat_interval: f32,
}

impl CommandPrompt {
    pub fn new() -> Self {
        CommandPrompt {
            input: String::new(),
            blink_timer: 0.0,
            blink_interval: 0.5,
            show_cursor: true,
            last_backspace_press_time: 0.0,
            backspace_press_count: 0,
            double_press_window: 0.3,
            backspace_held_time: 0.0,
            backspace_hold_delay: 0.3,
            backspace_repeat_interval: 0.05,
        }
    }

    pub fn add_char(&mut self, c: char) {
        self.input.push(c);
        self.reset_backspace_state();
    }

    pub fn backspace(&mut self) {
        if !self.input.is_empty() {
            self.input.pop();
        }
    }

    pub fn _clear(&mut self) {
        self.input.clear();
    }

    fn reset_backspace_state(&mut self) {
        self.last_backspace_press_time = 0.0;
        self.backspace_press_count = 0;
    }

    pub fn on_backspace_press(&mut self) {
        if self.last_backspace_press_time < self.double_press_window {
            self.backspace_press_count += 1;
            if self.backspace_press_count >= 2 {
                self.input.clear();
                self.reset_backspace_state();
                return;
            }
        } else {
            self.backspace_press_count = 1;
        }

        self.backspace();
        self.last_backspace_press_time = 0.0;
        self.backspace_held_time = 0.0;
    }

    pub fn update_backspace_hold(&mut self, delta_time: f32) {
        self.backspace_held_time += delta_time;
        self.last_backspace_press_time += delta_time;

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
    }
}
