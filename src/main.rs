mod biome;
mod block;
mod camera;
mod chunk;
mod command_prompt;
mod decoration;
mod gamemode;
mod input;
mod mesh_builder;
mod rng;
mod shader;
mod sky;
mod terrain;
mod text;
mod texture;
mod tree_generator;
mod ui;

use camera::Camera;
use cgmath::{Deg, Matrix, perspective};
use chunk::{CHUNK_SIZE, Chunk, ChunkMesh, ChunkPos, RENDER_DISTANCE};
use command_prompt::CommandPrompt;
use gl::types::*;
use glfw::Context;
use input::InputHandler;
use mesh_builder::MeshBuilder;
use shader::{FRAGMENT_SHADER, VERTEX_SHADER, compile_shader, link_program};
use sky::{SKY_FRAGMENT_SHADER, SKY_VERTEX_SHADER, Sky, get_wicked_time_of_day};
use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::CString;
use std::sync::{Arc, Mutex};
use terrain::TerrainGenerator;
use text::TextRenderer;
use texture::TextureAtlas;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tree_generator::TreeGenerator;
use ui::UserInterface;

struct ChunkGenerationRequest {
    pos: ChunkPos,
}

struct ChunkGenerationResult {
    pos: ChunkPos,
    chunk: Chunk,
}

struct ShaderUniforms {
    view_loc: GLint,
    proj_loc: GLint,
    sun_dir_loc: GLint,
    ambient_loc: GLint,
    sun_intensity_loc: GLint,
    wicked_time_loc: GLint,
}

struct Game {
    chunks: HashMap<ChunkPos, Chunk>,
    camera: Camera,
    shader_program: GLuint,
    sky_shader_program: GLuint,
    texture_atlas: TextureAtlas,
    input_handler: InputHandler,
    sky: Sky,
    chunk_request_tx: UnboundedSender<ChunkGenerationRequest>,
    chunk_result_rx: UnboundedReceiver<ChunkGenerationResult>,
    pending_chunks: HashSet<ChunkPos>,
    ui: UserInterface,
    shader_uniforms: ShaderUniforms,
    text_renderer: TextRenderer,
    command_prompt: CommandPrompt,
    command_prompt_visible: bool,
}

impl Game {
    pub fn new(shader_program: GLuint, sky_shader_program: GLuint) -> Result<Self, String> {
        let texture_atlas = TextureAtlas::new()?;
        let sky = Sky::new(sky_shader_program)?;
        let terrain_generator = Arc::new(TerrainGenerator::new(42));

        let shader_uniforms = unsafe {
            gl::UseProgram(shader_program);
            ShaderUniforms {
                view_loc: gl::GetUniformLocation(
                    shader_program,
                    CString::new("view").unwrap().as_ptr(),
                ),
                proj_loc: gl::GetUniformLocation(
                    shader_program,
                    CString::new("projection").unwrap().as_ptr(),
                ),
                sun_dir_loc: gl::GetUniformLocation(
                    shader_program,
                    CString::new("sunDirection").unwrap().as_ptr(),
                ),
                ambient_loc: gl::GetUniformLocation(
                    shader_program,
                    CString::new("ambientLight").unwrap().as_ptr(),
                ),
                sun_intensity_loc: gl::GetUniformLocation(
                    shader_program,
                    CString::new("sunIntensity").unwrap().as_ptr(),
                ),
                wicked_time_loc: gl::GetUniformLocation(
                    shader_program,
                    CString::new("wickedTime").unwrap().as_ptr(),
                ),
            }
        };

        let (chunk_request_tx, chunk_request_rx) = unbounded_channel::<ChunkGenerationRequest>();
        let (chunk_result_tx, chunk_result_rx) = unbounded_channel::<ChunkGenerationResult>();

        let chunk_request_rx = Arc::new(Mutex::new(chunk_request_rx));

        let num_workers = 8;
        for _ in 0..num_workers {
            let request_rx = Arc::clone(&chunk_request_rx);
            let result_tx = chunk_result_tx.clone();
            let terrain_gen = Arc::clone(&terrain_generator);

            tokio::task::spawn_blocking(move || {
                let mut local_tree_gen = TreeGenerator::new();

                loop {
                    let request = {
                        let mut rx = request_rx.lock().unwrap();
                        rx.blocking_recv()
                    };

                    match request {
                        Some(req) => {
                            let chunk = Chunk::new(req.pos, &terrain_gen, &mut local_tree_gen);
                            let _ = result_tx.send(ChunkGenerationResult {
                                pos: req.pos,
                                chunk,
                            });
                        }
                        None => break,
                    }
                }
            });
        }

        let text_renderer = TextRenderer::new()?;

        Ok(Game {
            chunks: HashMap::new(),
            camera: Camera::new(),
            shader_program,
            sky_shader_program,
            texture_atlas,
            input_handler: InputHandler::new(),
            sky,
            chunk_request_tx,
            chunk_result_rx,
            pending_chunks: HashSet::new(),
            ui: UserInterface::new(),
            shader_uniforms,
            text_renderer,
            command_prompt: CommandPrompt::new(),
            command_prompt_visible: false,
        })
    }

    fn cleanup_shader_programs(&self) {
        unsafe {
            gl::DeleteProgram(self.shader_program);
            gl::DeleteProgram(self.sky_shader_program);
        }
    }

    async fn update_chunks(&mut self) {
        let player_chunk_x = (self.camera.position.x / CHUNK_SIZE as f32).floor() as i32;
        let player_chunk_z = (self.camera.position.z / CHUNK_SIZE as f32).floor() as i32;

        for x in (player_chunk_x - RENDER_DISTANCE)..=(player_chunk_x + RENDER_DISTANCE) {
            for z in (player_chunk_z - RENDER_DISTANCE)..=(player_chunk_z + RENDER_DISTANCE) {
                let pos = ChunkPos { x, z };
                if !self.chunks.contains_key(&pos) && !self.pending_chunks.contains(&pos) {
                    let _ = self.chunk_request_tx.send(ChunkGenerationRequest { pos });
                    self.pending_chunks.insert(pos);
                }
            }
        }

        // Receive all available chunk results
        loop {
            match self.chunk_result_rx.try_recv() {
                Ok(result) => {
                    self.chunks.insert(result.pos, result.chunk);
                    self.pending_chunks.remove(&result.pos);
                }
                Err(_) => break,
            }
        }

        let unload_distance = RENDER_DISTANCE + 8;
        for (pos, chunk) in self.chunks.iter_mut() {
            let dx = (pos.x - player_chunk_x).abs();
            let dz = (pos.z - player_chunk_z).abs();

            if dx > unload_distance || dz > unload_distance {
                if chunk.mesh.is_some() {
                    chunk.mesh = None;
                }
            }
        }

        for x in (player_chunk_x - RENDER_DISTANCE)..=(player_chunk_x + RENDER_DISTANCE) {
            for z in (player_chunk_z - RENDER_DISTANCE)..=(player_chunk_z + RENDER_DISTANCE) {
                let pos = ChunkPos { x, z };
                if let Some(chunk) = self.chunks.get_mut(&pos) {
                    if chunk.mesh.is_none() {
                        let vertices = MeshBuilder::build_chunk_mesh(chunk, &self.texture_atlas);
                        if !vertices.is_empty() {
                            chunk.mesh = Some(ChunkMesh::new(&vertices));
                        }
                    }
                }
            }
        }

        self.ui
            .update_highlighted_block(self.camera.position, self.camera.front, &self.chunks);
    }

    fn render(&self, width: u32, height: u32, time: f32) {
        unsafe {
            gl::UseProgram(self.shader_program);

            let view = self.camera.get_view_matrix();
            let projection = perspective(Deg(45.0), width as f32 / height as f32, 0.1, 1000.0);

            gl::UniformMatrix4fv(self.shader_uniforms.view_loc, 1, gl::FALSE, view.as_ptr());
            gl::UniformMatrix4fv(
                self.shader_uniforms.proj_loc,
                1,
                gl::FALSE,
                projection.as_ptr(),
            );

            let sun_dir = self.sky.get_sun_direction(self.sky.day_night_cycle.time);
            gl::Uniform3f(
                self.shader_uniforms.sun_dir_loc,
                sun_dir.x,
                sun_dir.y,
                sun_dir.z,
            );
            gl::Uniform1f(
                self.shader_uniforms.ambient_loc,
                Sky::get_ambient_light(self.sky.day_night_cycle.time),
            );
            gl::Uniform1f(
                self.shader_uniforms.sun_intensity_loc,
                Sky::get_sun_intensity(self.sky.day_night_cycle.time),
            );
            gl::Uniform1f(
                self.shader_uniforms.wicked_time_loc,
                get_wicked_time_of_day(self.sky.day_night_cycle.time),
            );

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_atlas.texture_id);

            for chunk in self.chunks.values() {
                if let Some(ref mesh) = chunk.mesh {
                    mesh.render();
                }
            }

            let suncolor = [1.0, 1.0, 0.0, 1.0];
            let suncolor2 = [1.0, 1.0, 1.0, 1.0];
            let mooncolor = [0.5, 0.57, 0.65, 1.0];
            let mooncolor2 = [0.85, 0.875, 0.9, 1.0];

            self.sky.render(
                self.camera.position,
                &view,
                &projection,
                self.sky.day_night_cycle.time,
                suncolor,
                suncolor2,
                mooncolor,
                mooncolor2,
            );

            self.ui
                .render_highlight(&view, &projection, self.camera.position);
        }

        self.ui.render(width, height);

        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }

        if self.command_prompt_visible {
            self.command_prompt
                .render(&self.text_renderer, width, height, 2.0, time);
        }

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        self.cleanup_shader_programs();
    }
}

#[tokio::main]
async fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(1920, 1080, "Rust Voxel Engine", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let vertex_shader = compile_shader(VERTEX_SHADER, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(FRAGMENT_SHADER, gl::FRAGMENT_SHADER);
    let shader_program = link_program(vertex_shader, fragment_shader);

    let sky_vertex_shader = compile_shader(SKY_VERTEX_SHADER, gl::VERTEX_SHADER);
    let sky_fragment_shader = compile_shader(SKY_FRAGMENT_SHADER, gl::FRAGMENT_SHADER);
    let sky_shader_program = link_program(sky_vertex_shader, sky_fragment_shader);

    let mut game =
        Game::new(shader_program, sky_shader_program).expect("Failed to initialize game");
    let mut last_frame = glfw.get_time() as f32;

    while !window.should_close() {
        let current_frame = glfw.get_time() as f32;
        let delta_time = current_frame - last_frame;
        last_frame = current_frame;

        glfw.poll_events();
        InputHandler::process_events(
            &mut window,
            &events,
            &mut game.input_handler,
            &mut game.camera,
            &mut game.ui,
            &mut game.chunks,
            &mut game.command_prompt,
            &mut game.command_prompt_visible,
            &mut game.sky,
            delta_time,
        );

        game.sky.day_night_cycle.update(delta_time);
        game.command_prompt.update(delta_time);
        game.update_chunks().await;

        let sky_color = Sky::get_sky_color(game.sky.day_night_cycle.time);
        unsafe {
            gl::ClearColor(sky_color[0], sky_color[1], sky_color[2], sky_color[3]);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let (width, height) = window.get_size();
        game.render(width as u32, height as u32, current_frame);

        window.swap_buffers();
    }
}
