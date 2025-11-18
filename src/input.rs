use crate::block::BlockType;
use crate::block::Rotation;
use crate::camera::Camera;
use crate::chunk::{Chunk, ChunkPos};
use crate::command::CommandHandler;
use crate::command_prompt::CommandPrompt;
use crate::gamemode::GameMode;
use crate::sky::Sky;
use crate::ui::UserInterface;
use cgmath::InnerSpace;
use glfw::{Action, Key, MouseButton, Window, WindowEvent};
use std::collections::HashMap;
use std::sync::mpsc::Receiver;

pub enum CommandPromptAction {
    None,
    Submitted(String),
}

pub struct InputHandler {
    pub last_mouse_x: f64,
    pub last_mouse_y: f64,
    pub first_mouse: bool,
    pub current_block: BlockType,
    pub current_rotation: crate::block::Rotation,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            last_mouse_x: 400.0,
            last_mouse_y: 300.0,
            first_mouse: true,
            current_block: BlockType::Stone,
            current_rotation: crate::block::Rotation { x: 0, y: 0, z: 0 },
        }
    }

    pub fn handle_keyboard_input(
        window: &mut Window,
        camera: &mut Camera,
        sky: &mut Sky,
        delta_time: f32,
        command_prompt_visible: bool,
    ) {
        if command_prompt_visible {
            return;
        }

        let mut speed = 15.0 * delta_time;

        if window.get_key(Key::LeftControl) == Action::Press {
            speed *= 2.0;
        }

        let right = camera.front.cross(camera.up).normalize();

        if window.get_key(Key::W) == Action::Press {
            camera.position += camera.forward * speed;
        }
        if window.get_key(Key::S) == Action::Press {
            camera.position -= camera.forward * speed;
        }
        if window.get_key(Key::A) == Action::Press {
            camera.position -= right * speed;
        }
        if window.get_key(Key::D) == Action::Press {
            camera.position += right * speed;
        }
        if window.get_key(Key::Space) == Action::Press {
            camera.position.y += speed;
        }
        if window.get_key(Key::LeftShift) == Action::Press {
            camera.position.y -= speed;
        }

        sky.day_night_cycle.fast_forward = window.get_key(Key::T) == Action::Press;
    }

    pub fn handle_mouse_movement(&mut self, xpos: f64, ypos: f64, camera: &mut Camera) {
        if self.first_mouse {
            self.last_mouse_x = xpos;
            self.last_mouse_y = ypos;
            self.first_mouse = false;
        }

        let xoffset = (xpos - self.last_mouse_x) as f32 * 0.1;
        let yoffset = (self.last_mouse_y - ypos) as f32 * 0.1;

        self.last_mouse_x = xpos;
        self.last_mouse_y = ypos;

        camera.yaw += xoffset;
        camera.pitch += yoffset;

        camera.pitch = camera.pitch.clamp(-89.0, 89.0);
        camera.update_vectors();
    }

    pub fn handle_window_event(
        event: &WindowEvent,
        input_handler: &mut Self,
        camera: &mut Camera,
        ui: &mut UserInterface,
        chunks: &mut HashMap<ChunkPos, Chunk>,
    ) {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                ui.mode = GameMode::Normal;
            }
            WindowEvent::Key(Key::Num1, _, Action::Press, _) => {
                ui.mode = GameMode::Normal;
            }
            WindowEvent::Key(Key::Num2, _, Action::Press, _) => {
                ui.mode = GameMode::Insert;
            }
            WindowEvent::Key(Key::Num3, _, Action::Press, _) => {
                ui.mode = GameMode::Delete;
            }
            WindowEvent::MouseButton(MouseButton::Button2, Action::Press, _) => {
                if ui.mode == GameMode::Insert {
                    input_handler.place_block(ui, chunks);
                }
            }
            WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _) => {
                if ui.mode == GameMode::Delete {
                    input_handler.delete_block(ui, chunks);
                }
            }
            WindowEvent::CursorPos(xpos, ypos) => {
                input_handler.handle_mouse_movement(*xpos, *ypos, camera);
            }
            _ => {}
        }
    }

    fn place_block(&self, ui: &UserInterface, chunks: &mut HashMap<ChunkPos, Chunk>) {
        use crate::chunk::CHUNK_SIZE;

        if let Some(block_pos) = ui.highlighted_block {
            let chunk_x = (block_pos.x as f32 / CHUNK_SIZE as f32).floor() as i32;
            let chunk_z = (block_pos.z as f32 / CHUNK_SIZE as f32).floor() as i32;
            let chunk_pos = ChunkPos {
                x: chunk_x,
                z: chunk_z,
            };

            let local_x = block_pos.x - chunk_x * CHUNK_SIZE;
            let local_z = block_pos.z - chunk_z * CHUNK_SIZE;

            if let Some(chunk) = chunks.get_mut(&chunk_pos) {
                chunk.set_block(
                    local_x,
                    block_pos.y,
                    local_z,
                    self.current_block,
                    self.current_rotation.clone(),
                );
            }

            if local_x == 0 {
                if let Some(adj_chunk) = chunks.get_mut(&ChunkPos {
                    x: chunk_x - 1,
                    z: chunk_z,
                }) {
                    adj_chunk.mesh = None;
                }
            }
            if local_x == CHUNK_SIZE - 1 {
                if let Some(adj_chunk) = chunks.get_mut(&ChunkPos {
                    x: chunk_x + 1,
                    z: chunk_z,
                }) {
                    adj_chunk.mesh = None;
                }
            }
            if local_z == 0 {
                if let Some(adj_chunk) = chunks.get_mut(&ChunkPos {
                    x: chunk_x,
                    z: chunk_z - 1,
                }) {
                    adj_chunk.mesh = None;
                }
            }
            if local_z == CHUNK_SIZE - 1 {
                if let Some(adj_chunk) = chunks.get_mut(&ChunkPos {
                    x: chunk_x,
                    z: chunk_z + 1,
                }) {
                    adj_chunk.mesh = None;
                }
            }
        }
    }

    fn delete_block(&self, ui: &UserInterface, chunks: &mut HashMap<ChunkPos, Chunk>) {
        use crate::block::BlockType;
        use crate::chunk::CHUNK_SIZE;

        if let Some(block_pos) = ui.highlighted_block {
            let chunk_x = (block_pos.x as f32 / CHUNK_SIZE as f32).floor() as i32;
            let chunk_z = (block_pos.z as f32 / CHUNK_SIZE as f32).floor() as i32;
            let chunk_pos = ChunkPos {
                x: chunk_x,
                z: chunk_z,
            };

            let local_x = block_pos.x - chunk_x * CHUNK_SIZE;
            let local_z = block_pos.z - chunk_z * CHUNK_SIZE;

            if let Some(chunk) = chunks.get_mut(&chunk_pos) {
                chunk.set_block(
                    local_x,
                    block_pos.y,
                    local_z,
                    BlockType::Air,
                    crate::block::Rotation { x: 0, y: 0, z: 0 },
                );
            }

            if local_x == 0 {
                if let Some(adj_chunk) = chunks.get_mut(&ChunkPos {
                    x: chunk_x - 1,
                    z: chunk_z,
                }) {
                    adj_chunk.mesh = None;
                }
            }
            if local_x == CHUNK_SIZE - 1 {
                if let Some(adj_chunk) = chunks.get_mut(&ChunkPos {
                    x: chunk_x + 1,
                    z: chunk_z,
                }) {
                    adj_chunk.mesh = None;
                }
            }
            if local_z == 0 {
                if let Some(adj_chunk) = chunks.get_mut(&ChunkPos {
                    x: chunk_x,
                    z: chunk_z - 1,
                }) {
                    adj_chunk.mesh = None;
                }
            }
            if local_z == CHUNK_SIZE - 1 {
                if let Some(adj_chunk) = chunks.get_mut(&ChunkPos {
                    x: chunk_x,
                    z: chunk_z + 1,
                }) {
                    adj_chunk.mesh = None;
                }
            }
        }
    }

    pub fn handle_command_prompt_event(
        event: &WindowEvent,
        command_prompt: &mut CommandPrompt,
        input_handler: &mut Self,
        sky: &mut Sky,
    ) -> CommandPromptAction {
        match event {
            WindowEvent::Key(Key::Slash, _, Action::Press, _) => {
                command_prompt.toggle();
                CommandPromptAction::None
            }
            WindowEvent::Key(Key::Up, _, Action::Press, _) => {
                command_prompt.move_suggestion_up();
                CommandPromptAction::None
            }
            WindowEvent::Key(Key::Down, _, Action::Press, _) => {
                command_prompt.move_suggestion_down();
                CommandPromptAction::None
            }
            WindowEvent::Key(Key::Right, _, Action::Press, _) => {
                command_prompt.apply_suggestion();
                CommandPromptAction::None
            }
            WindowEvent::Key(Key::Enter, _, Action::Press, _) => {
                let input = command_prompt.input.clone();
                let result = CommandHandler::execute(&input);

                match result {
                    crate::command::CommandResult::Success(msg) => {
                        let trimmed = input.trim();
                        if trimmed.starts_with("use") {
                            if let Ok(block_type) = std::str::FromStr::from_str(
                                trimmed
                                    .strip_prefix("use")
                                    .unwrap_or("")
                                    .strip_suffix(";")
                                    .unwrap_or("")
                                    .trim(),
                            ) {
                                input_handler.current_block = block_type;
                            }
                        }
                        command_prompt.set_message(msg);
                    }
                    crate::command::CommandResult::Error(msg) => {
                        command_prompt.set_message(msg);
                    }
                    crate::command::CommandResult::TimeChange(time_change) => {
                        use crate::command::TimeChange;
                        match time_change {
                            TimeChange::SetTime(time) => {
                                sky.day_night_cycle.time = time;
                                command_prompt.set_message(format!("Time set to: {}", time));
                            }
                            TimeChange::ToggleCycle => {
                                sky.day_night_cycle.cycle_enabled =
                                    !sky.day_night_cycle.cycle_enabled;
                                let state = if sky.day_night_cycle.cycle_enabled {
                                    "enabled"
                                } else {
                                    "disabled"
                                };
                                command_prompt.set_message(format!("Day-night cycle {}", state));
                            }
                        }
                    }
                    crate::command::CommandResult::RotationChange(rotation) => {
                        input_handler.current_rotation = crate::block::Rotation {
                            x: rotation.x,
                            y: rotation.y,
                            z: rotation.z,
                        };
                        command_prompt.set_message(format!(
                            "Block rotation set to: x={}, y={}, z={}",
                            rotation.x, rotation.y, rotation.z
                        ));
                    }
                }

                command_prompt.reset();
                CommandPromptAction::Submitted(input)
            }
            WindowEvent::Key(Key::Backspace, _, Action::Press, _) => {
                command_prompt.on_backspace_press();
                command_prompt.update_suggestions();
                CommandPromptAction::None
            }
            WindowEvent::Key(Key::Delete, _, Action::Press, _) => {
                command_prompt.clear();
                command_prompt.update_suggestions();
                CommandPromptAction::None
            }
            WindowEvent::Char(c) => {
                if command_prompt.input.len() < 200 {
                    command_prompt.add_char(*c);
                    command_prompt.update_suggestions();
                }
                CommandPromptAction::None
            }
            _ => CommandPromptAction::None,
        }
    }

    pub fn process_events(
        window: &mut Window,
        events: &Receiver<(f64, glfw::WindowEvent)>,
        input_handler: &mut Self,
        camera: &mut Camera,
        ui: &mut UserInterface,
        chunks: &mut HashMap<ChunkPos, Chunk>,
        command_prompt: &mut CommandPrompt,
        command_prompt_visible: &mut bool,
        sky: &mut Sky,
        delta_time: f32,
    ) {
        for (_, event) in glfw::flush_messages(events) {
            if *command_prompt_visible {
                match Self::handle_command_prompt_event(&event, command_prompt, input_handler, sky)
                {
                    CommandPromptAction::Submitted(_) => {
                        *command_prompt_visible = false;
                    }
                    CommandPromptAction::None => {}
                }
            }

            match event {
                WindowEvent::Key(Key::Slash, _, Action::Press, _) => {
                    *command_prompt_visible = !*command_prompt_visible;
                    command_prompt.reset();
                    if *command_prompt_visible {
                        command_prompt.update_suggestions();
                    }
                }
                _ => {
                    if !*command_prompt_visible {
                        Self::handle_window_event(&event, input_handler, camera, ui, chunks);
                    }
                }
            }
        }

        if !*command_prompt_visible {
            Self::handle_keyboard_input(window, camera, sky, delta_time, *command_prompt_visible);
        }

        if *command_prompt_visible {
            if window.get_key(Key::Backspace) == Action::Press {
                command_prompt.update_backspace_hold(delta_time);
            } else {
                command_prompt.on_backspace_release();
            }
        }

        window.set_char_polling(*command_prompt_visible);
    }
}
