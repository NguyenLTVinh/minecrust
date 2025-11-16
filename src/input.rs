use crate::camera::Camera;
use crate::chunk::{Chunk, ChunkPos};
use crate::gamemode::GameMode;
use crate::sky::Sky;
use crate::ui::UserInterface;
use cgmath::InnerSpace;
use glfw::{Action, Key, MouseButton, Window, WindowEvent};
use std::collections::HashMap;

pub struct InputHandler {
    pub last_mouse_x: f64,
    pub last_mouse_y: f64,
    pub first_mouse: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            last_mouse_x: 400.0,
            last_mouse_y: 300.0,
            first_mouse: true,
        }
    }

    pub fn handle_keyboard_input(
        window: &mut Window,
        camera: &mut Camera,
        sky: &mut Sky,
        delta_time: f32,
    ) {
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
                chunk.set_block(local_x, block_pos.y, local_z, BlockType::Stone);
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
                chunk.set_block(local_x, block_pos.y, local_z, BlockType::Air);
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
}
