use cgmath::{Deg, InnerSpace, Matrix, Matrix4, Point3, Vector3, perspective};
use gl::types::*;
use glfw::{Action, Context, Key};
use noise::{NoiseFn, Perlin};
use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::CString;
use std::mem;
use std::ptr;

// Block types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum BlockType {
    Air,
    Grass,
    Stone,
    Wood,
    Leaves,
}

impl BlockType {
    fn is_solid(&self) -> bool {
        !matches!(self, BlockType::Air)
    }

    fn get_color(&self) -> [f32; 3] {
        match self {
            BlockType::Air => [0.0, 0.0, 0.0],
            BlockType::Grass => [0.2, 0.8, 0.2],
            BlockType::Stone => [0.5, 0.5, 0.5],
            BlockType::Wood => [0.55, 0.35, 0.2],
            BlockType::Leaves => [0.1, 0.6, 0.1],
        }
    }
}

// Chunk constants
const CHUNK_SIZE: i32 = 16;
const CHUNK_HEIGHT: i32 = 64;
const RENDER_DISTANCE: i32 = 4;

// Chunk position
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ChunkPos {
    x: i32,
    z: i32,
}

// Chunk data structure
struct Chunk {
    blocks: Vec<BlockType>,
    mesh: Option<ChunkMesh>,
    pos: ChunkPos,
}

impl Chunk {
    pub fn new(pos: ChunkPos, perlin: &Perlin, tree_generator: &mut TreeGenerator) -> Self {
        let mut chunk = Chunk {
            blocks: vec![BlockType::Air; (CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE) as usize],
            mesh: None,
            pos,
        };

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = pos.x * CHUNK_SIZE + x;
                let world_z = pos.z * CHUNK_SIZE + z;
                let height = (perlin.get([world_x as f64 * 0.05, world_z as f64 * 0.05]) * 8.0
                    + 32.0) as i32;

                // Place blocks
                for y in 0..CHUNK_HEIGHT.min(height) {
                    let idx = Self::get_index(x, y, z);
                    chunk.blocks[idx] = if y == height - 1 {
                        BlockType::Grass
                    } else {
                        BlockType::Stone
                    };
                }

                // Tree generation
                let tree_noise =
                    perlin.get([world_x as f64 * 0.1 + 1000.0, world_z as f64 * 0.1 + 1000.0]);
                if tree_noise > 0.7 && height < CHUNK_HEIGHT - 8 {
                    if tree_generator.can_place_tree(world_x, world_z) {
                        let trunk_height =
                            tree_generator.random_trunk_height(perlin, world_x, world_z);
                        chunk.generate_tree(x, height, z, trunk_height);
                        tree_generator.register_tree(world_x, world_z);
                    }
                }
            }
        }
        chunk
    }

    fn get_index(x: i32, y: i32, z: i32) -> usize {
        (x + z * CHUNK_SIZE + y * CHUNK_SIZE * CHUNK_SIZE) as usize
    }

    fn get_block(&self, x: i32, y: i32, z: i32) -> BlockType {
        if x < 0 || x >= CHUNK_SIZE || y < 0 || y >= CHUNK_HEIGHT || z < 0 || z >= CHUNK_SIZE {
            return BlockType::Air;
        }
        self.blocks[Self::get_index(x, y, z)]
    }

    fn set_block(&mut self, x: i32, y: i32, z: i32, block: BlockType) {
        if x < 0 || x >= CHUNK_SIZE || y < 0 || y >= CHUNK_HEIGHT || z < 0 || z >= CHUNK_SIZE {
            return;
        }
        self.blocks[Self::get_index(x, y, z)] = block;
        self.mesh = None; // Invalidate mesh
    }

    fn generate_tree(&mut self, x: i32, base_y: i32, z: i32, trunk_height: i32) {
        let leaf_radius = 2;

        // Generate trunk
        for y in base_y..(base_y + trunk_height) {
            self.set_block(x, y, z, BlockType::Wood);
        }

        // Generate leaves
        let leaf_start = base_y + trunk_height - 2;
        for dy in 0..4 {
            let y = leaf_start + dy;
            let radius = if dy == 3 { 1 } else { leaf_radius };

            for dx in -radius..=radius {
                for dz in -radius..=radius {
                    let dist = (dx * dx + dz * dz) as f32;
                    if dist <= (radius * radius) as f32 {
                        let block = self.get_block(x + dx, y, z + dz);
                        if block == BlockType::Air {
                            self.set_block(x + dx, y, z + dz, BlockType::Leaves);
                        }
                    }
                }
            }
        }
    }
}

pub struct TreeGenerator {
    tree_positions: HashSet<(i32, i32)>,
}

impl TreeGenerator {
    pub fn new() -> Self {
        TreeGenerator {
            tree_positions: HashSet::new(),
        }
    }

    pub fn can_place_tree(&self, world_x: i32, world_z: i32) -> bool {
        // Check if tree is too close to chunk borders
        let local_x = world_x.rem_euclid(CHUNK_SIZE);
        let local_z = world_z.rem_euclid(CHUNK_SIZE);

        if local_x < 3 || local_x >= CHUNK_SIZE - 3 || local_z < 3 || local_z >= CHUNK_SIZE - 3 {
            return false;
        }

        // Check if tree is too close to other trees
        !self.tree_positions.iter().any(|&(tx, tz)| {
            let dx = (tx - world_x).abs();
            let dz = (tz - world_z).abs();
            dx < 5 && dz < 5
        })
    }

    pub fn register_tree(&mut self, world_x: i32, world_z: i32) {
        self.tree_positions.insert((world_x, world_z));
    }

    pub fn random_trunk_height(&self, perlin: &Perlin, world_x: i32, world_z: i32) -> i32 {
        4 + (perlin
            .get([world_x as f64 * 0.2, world_z as f64 * 0.2])
            .abs()
            * 3.0) as i32
    }
}

// Mesh data for rendering
struct ChunkMesh {
    vao: GLuint,
    vbo: GLuint,
    vertex_count: i32,
}

impl ChunkMesh {
    fn new(vertices: &[f32]) -> Self {
        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<f32>()) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Position attribute
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                9 * mem::size_of::<f32>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // Color attribute
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                9 * mem::size_of::<f32>() as GLsizei,
                (3 * mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            // Normal attribute (for lighting)
            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                9 * mem::size_of::<f32>() as GLsizei,
                (6 * mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(2);

            gl::BindVertexArray(0);
        }

        ChunkMesh {
            vao,
            vbo,
            vertex_count: (vertices.len() / 9) as i32,
        }
    }

    fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for ChunkMesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

// Generate mesh for a chunk (greedy meshing simplified)
fn generate_chunk_mesh(chunk: &Chunk) -> Vec<f32> {
    let mut vertices = Vec::new();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                let block = chunk.get_block(x, y, z);
                if !block.is_solid() {
                    continue;
                }

                let color = block.get_color();
                let wx = (chunk.pos.x * CHUNK_SIZE + x) as f32;
                let wy = y as f32;
                let wz = (chunk.pos.z * CHUNK_SIZE + z) as f32;

                // Check each face and only render if adjacent block is air
                let faces = [
                    (0, 1, 0, true),  // Top
                    (0, -1, 0, true), // Bottom
                    (0, 0, 1, true),  // Front
                    (0, 0, -1, true), // Back
                    (1, 0, 0, true),  // Right
                    (-1, 0, 0, true), // Left
                ];

                for (dx, dy, dz, _) in faces {
                    if !chunk.get_block(x + dx, y + dy, z + dz).is_solid() {
                        add_face(&mut vertices, wx, wy, wz, dx, dy, dz, color);
                    }
                }
            }
        }
    }

    vertices
}

fn add_face(
    vertices: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
    dx: i32,
    dy: i32,
    dz: i32,
    color: [f32; 3],
) {
    let normal = [dx as f32, dy as f32, dz as f32];
    let light = 0.6 + 0.4 * (dy as f32 * 0.5 + 0.5); // Simple lighting based on face direction
    let lit_color = [color[0] * light, color[1] * light, color[2] * light];

    let verts = match (dx, dy, dz) {
        (0, 1, 0) => vec![
            // Top
            x,
            y + 1.0,
            z,
            x,
            y + 1.0,
            z + 1.0,
            x + 1.0,
            y + 1.0,
            z + 1.0,
            x,
            y + 1.0,
            z,
            x + 1.0,
            y + 1.0,
            z + 1.0,
            x + 1.0,
            y + 1.0,
            z,
        ],
        (0, -1, 0) => vec![
            // Bottom
            x,
            y,
            z,
            x + 1.0,
            y,
            z,
            x + 1.0,
            y,
            z + 1.0,
            x,
            y,
            z,
            x + 1.0,
            y,
            z + 1.0,
            x,
            y,
            z + 1.0,
        ],
        (0, 0, 1) => vec![
            // Front
            x,
            y,
            z + 1.0,
            x,
            y + 1.0,
            z + 1.0,
            x + 1.0,
            y + 1.0,
            z + 1.0,
            x,
            y,
            z + 1.0,
            x + 1.0,
            y + 1.0,
            z + 1.0,
            x + 1.0,
            y,
            z + 1.0,
        ],
        (0, 0, -1) => vec![
            // Back
            x,
            y,
            z,
            x + 1.0,
            y,
            z,
            x + 1.0,
            y + 1.0,
            z,
            x,
            y,
            z,
            x + 1.0,
            y + 1.0,
            z,
            x,
            y + 1.0,
            z,
        ],
        (1, 0, 0) => vec![
            // Right
            x + 1.0,
            y,
            z,
            x + 1.0,
            y + 1.0,
            z,
            x + 1.0,
            y + 1.0,
            z + 1.0,
            x + 1.0,
            y,
            z,
            x + 1.0,
            y + 1.0,
            z + 1.0,
            x + 1.0,
            y,
            z + 1.0,
        ],
        (-1, 0, 0) => vec![
            // Left
            x,
            y,
            z,
            x,
            y,
            z + 1.0,
            x,
            y + 1.0,
            z + 1.0,
            x,
            y,
            z,
            x,
            y + 1.0,
            z + 1.0,
            x,
            y + 1.0,
            z,
        ],
        _ => vec![],
    };

    for i in (0..verts.len()).step_by(3) {
        vertices.extend_from_slice(&verts[i..i + 3]);
        vertices.extend_from_slice(&lit_color);
        vertices.extend_from_slice(&normal);
    }
}

// Camera
struct Camera {
    position: Point3<f32>,
    front: Vector3<f32>,
    up: Vector3<f32>,
    yaw: f32,
    pitch: f32,
    velocity: Vector3<f32>,
}

impl Camera {
    fn new() -> Self {
        Camera {
            position: Point3::new(0.0, 40.0, 0.0),
            front: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
            velocity: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    fn update_vectors(&mut self) {
        let front = Vector3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.front = front.normalize();
    }

    fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.position, self.position + self.front, self.up)
    }
}

// Shader compilation
fn compile_shader(src: &str, shader_type: GLenum) -> GLuint {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut _);
            panic!(
                "Shader compilation failed: {}",
                String::from_utf8_lossy(&buffer)
            );
        }

        shader
    }
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            panic!("Program linking failed");
        }

        gl::DeleteShader(vs);
        gl::DeleteShader(fs);

        program
    }
}

// Main game structure
struct Game {
    chunks: HashMap<ChunkPos, Chunk>,
    camera: Camera,
    perlin: Perlin,
    shader_program: GLuint,
    last_mouse_x: f64,
    last_mouse_y: f64,
    first_mouse: bool,
    tree_generator: TreeGenerator,
}

impl Game {
    pub fn new(shader_program: GLuint) -> Self {
        Game {
            chunks: HashMap::new(),
            camera: Camera::new(),
            perlin: Perlin::new(42),
            shader_program,
            last_mouse_x: 400.0,
            last_mouse_y: 300.0,
            first_mouse: true,
            tree_generator: TreeGenerator::new(),
        }
    }

    fn update_chunks(&mut self) {
        let player_chunk_x = (self.camera.position.x / CHUNK_SIZE as f32).floor() as i32;
        let player_chunk_z = (self.camera.position.z / CHUNK_SIZE as f32).floor() as i32;

        for x in (player_chunk_x - RENDER_DISTANCE)..=(player_chunk_x + RENDER_DISTANCE) {
            for z in (player_chunk_z - RENDER_DISTANCE)..=(player_chunk_z + RENDER_DISTANCE) {
                let pos = ChunkPos { x, z };
                if !self.chunks.contains_key(&pos) {
                    let chunk = Chunk::new(pos, &self.perlin, &mut self.tree_generator);
                    self.chunks.insert(pos, chunk);
                }
            }
        }

        for chunk in self.chunks.values_mut() {
            if chunk.mesh.is_none() {
                let vertices = generate_chunk_mesh(chunk);
                if !vertices.is_empty() {
                    chunk.mesh = Some(ChunkMesh::new(&vertices));
                }
            }
        }
    }

    fn handle_input(&mut self, window: &mut glfw::Window, delta_time: f32) {
        let speed = 10.0 * delta_time;
        let right = self.camera.front.cross(self.camera.up).normalize();

        if window.get_key(Key::W) == Action::Press {
            self.camera.position += self.camera.front * speed;
        }
        if window.get_key(Key::S) == Action::Press {
            self.camera.position -= self.camera.front * speed;
        }
        if window.get_key(Key::A) == Action::Press {
            self.camera.position -= right * speed;
        }
        if window.get_key(Key::D) == Action::Press {
            self.camera.position += right * speed;
        }
        if window.get_key(Key::Space) == Action::Press {
            self.camera.position.y += speed;
        }
        if window.get_key(Key::LeftShift) == Action::Press {
            self.camera.position.y -= speed;
        }
    }

    fn handle_mouse(&mut self, xpos: f64, ypos: f64) {
        if self.first_mouse {
            self.last_mouse_x = xpos;
            self.last_mouse_y = ypos;
            self.first_mouse = false;
        }

        let xoffset = (xpos - self.last_mouse_x) as f32 * 0.1;
        let yoffset = (self.last_mouse_y - ypos) as f32 * 0.1;

        self.last_mouse_x = xpos;
        self.last_mouse_y = ypos;

        self.camera.yaw += xoffset;
        self.camera.pitch += yoffset;

        self.camera.pitch = self.camera.pitch.clamp(-89.0, 89.0);
        self.camera.update_vectors();
    }

    fn render(&self, width: u32, height: u32) {
        unsafe {
            gl::UseProgram(self.shader_program);

            let view = self.camera.get_view_matrix();
            let projection = perspective(Deg(45.0), width as f32 / height as f32, 0.1, 1000.0);

            let view_loc =
                gl::GetUniformLocation(self.shader_program, CString::new("view").unwrap().as_ptr());
            let proj_loc = gl::GetUniformLocation(
                self.shader_program,
                CString::new("projection").unwrap().as_ptr(),
            );

            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr());
            gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, projection.as_ptr());

            for chunk in self.chunks.values() {
                if let Some(ref mesh) = chunk.mesh {
                    mesh.render();
                }
            }
        }
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(1280, 720, "Rust Voxel Engine", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        // gl::Enable(gl::CULL_FACE);
        // gl::CullFace(gl::BACK);
    }

    let vertex_shader = compile_shader(VERTEX_SHADER, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(FRAGMENT_SHADER, gl::FRAGMENT_SHADER);
    let shader_program = link_program(vertex_shader, fragment_shader);

    let mut game = Game::new(shader_program);
    let mut last_frame = glfw.get_time() as f32;

    while !window.should_close() {
        let current_frame = glfw.get_time() as f32;
        let delta_time = current_frame - last_frame;
        last_frame = current_frame;

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    game.handle_mouse(xpos, ypos);
                }
                _ => {}
            }
        }

        game.handle_input(&mut window, delta_time);
        game.update_chunks();

        unsafe {
            gl::ClearColor(0.53, 0.81, 0.92, 1.0); // Sky blue
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let (width, height) = window.get_size();
        game.render(width as u32, height as u32);

        window.swap_buffers();
    }
}

const VERTEX_SHADER: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec3 aNormal;

out vec3 FragColor;
out vec3 Normal;

uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * vec4(aPos, 1.0);
    FragColor = aColor;
    Normal = aNormal;
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 330 core
in vec3 FragColor;
in vec3 Normal;

out vec4 color;

void main() {
    vec3 lightDir = normalize(vec3(0.5, 1.0, 0.3));
    float diff = max(dot(Normal, lightDir), 0.3);
    vec3 result = FragColor * diff;
    color = vec4(result, 1.0);
}
"#;
