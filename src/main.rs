mod sky;

use cgmath::{Deg, InnerSpace, Matrix, Matrix4, Point3, Vector3, perspective};
use gl::types::*;
use glfw::{Action, Context, Key};
use image::{GenericImageView, RgbaImage};
use noise::{NoiseFn, Perlin};
use sky::{SKY_FRAGMENT_SHADER, SKY_VERTEX_SHADER, Sky};
use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::CString;
use std::mem;
use std::ptr;

// Texture atlas configuration
const ATLAS_SIZE: u32 = 32; // 2x4 grid = 32x64 pixels
const TEXTURE_SIZE: u32 = 16;

#[derive(Clone, Copy)]
struct TextureCoords {
    u_min: f32,
    v_min: f32,
    u_max: f32,
    v_max: f32,
}

impl TextureCoords {
    fn new(index: usize, atlas_width: u32, atlas_height: u32) -> Self {
        let textures_per_row = (atlas_width / TEXTURE_SIZE) as usize;
        let row = index / textures_per_row;
        let col = index % textures_per_row;

        let u_min = (col * TEXTURE_SIZE as usize) as f32 / atlas_width as f32;
        let v_min = (row * TEXTURE_SIZE as usize) as f32 / atlas_height as f32;
        let u_max = ((col + 1) * TEXTURE_SIZE as usize) as f32 / atlas_width as f32;
        let v_max = ((row + 1) * TEXTURE_SIZE as usize) as f32 / atlas_height as f32;

        TextureCoords {
            u_min,
            v_min,
            u_max,
            v_max,
        }
    }
}

// Texture atlas manager
struct TextureAtlas {
    texture_id: GLuint,
    grass_color: [f32; 3],
    foliage_color: [f32; 3],
    atlas_width: u32,
    atlas_height: u32,
}

impl TextureAtlas {
    fn new() -> Result<Self, String> {
        // Load all textures
        let stone = image::open("textures/block/stone.png")
            .map_err(|e| format!("Failed to load stone.png: {}", e))?;
        let dirt = image::open("textures/block/dirt.png")
            .map_err(|e| format!("Failed to load dirt.png: {}", e))?;
        let grass_side = image::open("textures/block/grass_block_side.png")
            .map_err(|e| format!("Failed to load grass_block_side.png: {}", e))?;
        let grass_top = image::open("textures/block/grass_block_top.png")
            .map_err(|e| format!("Failed to load grass_block_top.png: {}", e))?;
        let oak_log = image::open("textures/block/oak_log.png")
            .map_err(|e| format!("Failed to load oak_log.png: {}", e))?;
        let oak_log_top = image::open("textures/block/oak_log_top.png")
            .map_err(|e| format!("Failed to load oak_log_top.png: {}", e))?;
        let oak_leaves = image::open("textures/block/oak_leaves.png")
            .map_err(|e| format!("Failed to load oak_leaves.png: {}", e))?;

        // Create atlas (2x4 grid for 7 textures)
        let atlas_width = TEXTURE_SIZE * 2;
        let atlas_height = TEXTURE_SIZE * 4;
        let mut atlas = RgbaImage::new(atlas_width, atlas_height);

        // Copy textures into atlas
        let textures = vec![
            stone,
            dirt,
            grass_side,
            grass_top,
            oak_log,
            oak_log_top,
            oak_leaves,
        ];
        for (i, tex) in textures.iter().enumerate() {
            let tex = tex.to_rgba8();
            let row = i / 2;
            let col = i % 2;
            let x_offset = col as u32 * TEXTURE_SIZE;
            let y_offset = row as u32 * TEXTURE_SIZE;

            for y in 0..TEXTURE_SIZE {
                for x in 0..TEXTURE_SIZE {
                    let pixel = tex.get_pixel(x, y);
                    atlas.put_pixel(x_offset + x, y_offset + y, *pixel);
                }
            }
        }

        // Load colormaps
        let grass_color = Self::load_colormap_sample("textures/colormap/grass.png")?;
        let foliage_color = Self::load_colormap_sample("textures/colormap/foliage.png")?;

        // Create OpenGL texture
        let texture_id = unsafe {
            let mut texture = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                atlas_width as i32,
                atlas_height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                atlas.as_raw().as_ptr() as *const _,
            );

            texture
        };

        Ok(TextureAtlas {
            texture_id,
            grass_color,
            foliage_color,
            atlas_width,
            atlas_height,
        })
    }

    fn load_colormap_sample(path: &str) -> Result<[f32; 3], String> {
        let img = image::open(path).map_err(|e| format!("Failed to load {}: {}", path, e))?;
        let img = img.to_rgba8();
        let (width, height) = img.dimensions();

        let x = width / 2;
        let y = height / 2;
        let pixel = img.get_pixel(x, y);

        Ok([
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        ])
    }

    fn get_tex_coords(&self, block: BlockType, face: FaceDirection) -> TextureCoords {
        let index = match block {
            BlockType::Stone => 0,
            BlockType::Grass => match face {
                FaceDirection::Top => 3,
                FaceDirection::Bottom => 1,
                _ => 2,
            },
            BlockType::Wood => match face {
                FaceDirection::Top | FaceDirection::Bottom => 5,
                _ => 4,
            },
            BlockType::Leaves => 6,
            BlockType::Air => 0,
        };
        TextureCoords::new(index, self.atlas_width, self.atlas_height)
    }

    fn get_tint(&self, block: BlockType) -> [f32; 3] {
        match block {
            BlockType::Grass => self.grass_color,
            BlockType::Leaves => self.foliage_color,
            _ => [1.0, 1.0, 1.0],
        }
    }
}

impl Drop for TextureAtlas {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FaceDirection {
    Top,
    Bottom,
    Front,
    Back,
    Right,
    Left,
}

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

const CHUNK_SIZE: i32 = 16;
const CHUNK_HEIGHT: i32 = 64;
const RENDER_DISTANCE: i32 = 16;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ChunkPos {
    x: i32,
    z: i32,
}

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

            // Vertex format: position(3) + uv(2) + tint(3) + normal(3) = 11 floats
            let stride = 11 * mem::size_of::<f32>() as GLsizei;

            // Position attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            // UV attribute
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            // Tint attribute
            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (5 * mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(2);

            // Normal attribute (for lighting)
            gl::VertexAttribPointer(
                3,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (8 * mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(3);

            gl::BindVertexArray(0);
        }

        ChunkMesh {
            vao,
            vbo,
            vertex_count: (vertices.len() / 11) as i32,
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

// Generate mesh for a chunk with texture atlas
fn generate_chunk_mesh(chunk: &Chunk, atlas: &TextureAtlas) -> Vec<f32> {
    let mut vertices = Vec::new();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                let block = chunk.get_block(x, y, z);
                if !block.is_solid() {
                    continue;
                }

                let wx = (chunk.pos.x * CHUNK_SIZE + x) as f32;
                let wy = y as f32;
                let wz = (chunk.pos.z * CHUNK_SIZE + z) as f32;
                let tint = atlas.get_tint(block);

                // Check each face and only render if adjacent block is air
                let faces = [
                    (FaceDirection::Top, 0, 1, 0),
                    (FaceDirection::Bottom, 0, -1, 0),
                    (FaceDirection::Front, 0, 0, 1),
                    (FaceDirection::Back, 0, 0, -1),
                    (FaceDirection::Right, 1, 0, 0),
                    (FaceDirection::Left, -1, 0, 0),
                ];

                for (face_dir, dx, dy, dz) in faces {
                    if !chunk.get_block(x + dx, y + dy, z + dz).is_solid() {
                        let tex_coords = atlas.get_tex_coords(block, face_dir);
                        add_face(&mut vertices, wx, wy, wz, dx, dy, dz, tex_coords, tint);
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
    tex: TextureCoords,
    tint: [f32; 3],
) {
    let normal = [dx as f32, dy as f32, dz as f32];

    // Apply different UV coordinates based on face direction
    // Flip V coordinates for south, east, and west faces to fix upside-down textures
    let uvs = match (dx, dy, dz) {
        (-1, 0, 0) => [
            // West - flip V coordinates
            [tex.u_max, tex.v_max],
            [tex.u_min, tex.v_max],
            [tex.u_min, tex.v_min],
            [tex.u_max, tex.v_max],
            [tex.u_min, tex.v_min],
            [tex.u_max, tex.v_min],
        ],
        (1, 0, 0) => [
            // East - flip V coordinates
            [tex.u_min, tex.v_max],
            [tex.u_min, tex.v_min],
            [tex.u_max, tex.v_min],
            [tex.u_min, tex.v_max],
            [tex.u_max, tex.v_min],
            [tex.u_max, tex.v_max],
        ],
        (0, 0, -1) => [
            // North - correct orientation (original, unchanged)
            [tex.u_min, tex.v_max],
            [tex.u_max, tex.v_max],
            [tex.u_max, tex.v_min],
            [tex.u_min, tex.v_max],
            [tex.u_max, tex.v_min],
            [tex.u_min, tex.v_min],
        ],
        (0, 0, 1) => [
            // South - flip V coordinates
            [tex.u_min, tex.v_max],
            [tex.u_min, tex.v_min],
            [tex.u_max, tex.v_min],
            [tex.u_min, tex.v_max],
            [tex.u_max, tex.v_min],
            [tex.u_max, tex.v_max],
        ],
        _ => [
            [tex.u_min, tex.v_min],
            [tex.u_min, tex.v_max],
            [tex.u_max, tex.v_max],
            [tex.u_min, tex.v_min],
            [tex.u_max, tex.v_max],
            [tex.u_max, tex.v_min],
        ],
    };

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

    for (i, pos_idx) in (0..verts.len()).step_by(3).enumerate() {
        vertices.extend_from_slice(&verts[pos_idx..pos_idx + 3]);
        vertices.extend_from_slice(&uvs[i]);
        vertices.extend_from_slice(&tint);
        vertices.extend_from_slice(&normal);
    }
}

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

struct DayNightCycle {
    time: f32,
    tick_speed: f32,
    fast_forward: bool,
}

impl DayNightCycle {
    fn new() -> Self {
        DayNightCycle {
            time: 0.25,
            tick_speed: 0.01,
            fast_forward: false,
        }
    }

    fn update(&mut self, delta_time: f32) {
        let speed = if self.fast_forward {
            self.tick_speed * 10.0 // 10x speed when fast forwarding
        } else {
            self.tick_speed
        };

        self.time += speed * delta_time;
        if self.time > 1.0 {
            self.time -= 1.0;
        }
    }
}

struct Game {
    chunks: HashMap<ChunkPos, Chunk>,
    camera: Camera,
    perlin: Perlin,
    shader_program: GLuint,
    sky_shader_program: GLuint,
    texture_atlas: TextureAtlas,
    last_mouse_x: f64,
    last_mouse_y: f64,
    first_mouse: bool,
    tree_generator: TreeGenerator,
    day_night_cycle: DayNightCycle,
    sky: Sky,
}

impl Game {
    pub fn new(shader_program: GLuint, sky_shader_program: GLuint) -> Result<Self, String> {
        let texture_atlas = TextureAtlas::new()?;
        let sky = Sky::new(sky_shader_program)?;

        Ok(Game {
            chunks: HashMap::new(),
            camera: Camera::new(),
            perlin: Perlin::new(02252005),
            shader_program,
            sky_shader_program,
            texture_atlas,
            last_mouse_x: 400.0,
            last_mouse_y: 300.0,
            first_mouse: true,
            tree_generator: TreeGenerator::new(),
            day_night_cycle: DayNightCycle::new(),
            sky,
        })
    }

    fn cleanup_shader_programs(&self) {
        unsafe {
            gl::DeleteProgram(self.shader_program);
            gl::DeleteProgram(self.sky_shader_program);
        }
    }

    fn update_chunks(&mut self) {
        let player_chunk_x = (self.camera.position.x / CHUNK_SIZE as f32).floor() as i32;
        let player_chunk_z = (self.camera.position.z / CHUNK_SIZE as f32).floor() as i32;

        // Generate new chunks within render distance
        for x in (player_chunk_x - RENDER_DISTANCE)..=(player_chunk_x + RENDER_DISTANCE) {
            for z in (player_chunk_z - RENDER_DISTANCE)..=(player_chunk_z + RENDER_DISTANCE) {
                let pos = ChunkPos { x, z };
                if !self.chunks.contains_key(&pos) {
                    let chunk = Chunk::new(pos, &self.perlin, &mut self.tree_generator);
                    self.chunks.insert(pos, chunk);
                }
            }
        }

        // Unload meshes for chunks that are too far from player (memory leak fix)
        // Keep the block data so player builds are preserved
        // Use a larger distance before unloading to avoid thrashing
        let unload_distance = RENDER_DISTANCE + 8;
        for (pos, chunk) in self.chunks.iter_mut() {
            let dx = (pos.x - player_chunk_x).abs();
            let dz = (pos.z - player_chunk_z).abs();

            // If chunk is too far, unload its mesh to free GPU memory
            if dx > unload_distance || dz > unload_distance {
                if chunk.mesh.is_some() {
                    chunk.mesh = None; // Drop will automatically free GPU resources
                }
            }
        }

        // Generate/regenerate meshes for chunks within render distance
        for x in (player_chunk_x - RENDER_DISTANCE)..=(player_chunk_x + RENDER_DISTANCE) {
            for z in (player_chunk_z - RENDER_DISTANCE)..=(player_chunk_z + RENDER_DISTANCE) {
                let pos = ChunkPos { x, z };
                if let Some(chunk) = self.chunks.get_mut(&pos) {
                    if chunk.mesh.is_none() {
                        let vertices = generate_chunk_mesh(chunk, &self.texture_atlas);
                        if !vertices.is_empty() {
                            chunk.mesh = Some(ChunkMesh::new(&vertices));
                        }
                    }
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

        self.day_night_cycle.fast_forward = window.get_key(Key::T) == Action::Press;
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
            let sun_dir_loc = gl::GetUniformLocation(
                self.shader_program,
                CString::new("sunDirection").unwrap().as_ptr(),
            );
            let ambient_loc = gl::GetUniformLocation(
                self.shader_program,
                CString::new("ambientLight").unwrap().as_ptr(),
            );
            let sun_intensity_loc = gl::GetUniformLocation(
                self.shader_program,
                CString::new("sunIntensity").unwrap().as_ptr(),
            );

            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr());
            gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, projection.as_ptr());

            // Pass sun direction, ambient light, and sun intensity to shader using Sky methods
            let sun_dir = self.sky.get_sun_direction(self.day_night_cycle.time);
            gl::Uniform3f(sun_dir_loc, sun_dir.x, sun_dir.y, sun_dir.z);
            gl::Uniform1f(
                ambient_loc,
                Sky::get_ambient_light(self.day_night_cycle.time),
            );
            gl::Uniform1f(
                sun_intensity_loc,
                Sky::get_sun_intensity(self.day_night_cycle.time),
            );

            // Bind texture atlas
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_atlas.texture_id);

            for chunk in self.chunks.values() {
                if let Some(ref mesh) = chunk.mesh {
                    mesh.render();
                }
            }

            // Calculate sun and moon colors based on time of day
            let suncolor = [1.0, 1.0, 0.0, 1.0]; // Yellow sun
            let suncolor2 = [1.0, 1.0, 1.0, 1.0]; // White center
            let mooncolor = [0.5, 0.57, 0.65, 1.0]; // Bluish moon
            let mooncolor2 = [0.85, 0.875, 0.9, 1.0]; // Lighter moon center

            self.sky.render(
                self.camera.position,
                &view,
                &projection,
                self.day_night_cycle.time,
                suncolor,
                suncolor2,
                mooncolor,
                mooncolor2,
            );
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        self.cleanup_shader_programs();
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
        game.day_night_cycle.update(delta_time);
        game.update_chunks();

        let sky_color = Sky::get_sky_color(game.day_night_cycle.time);
        unsafe {
            gl::ClearColor(sky_color[0], sky_color[1], sky_color[2], sky_color[3]);
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
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec3 aTint;
layout (location = 3) in vec3 aNormal;

out vec2 TexCoord;
out vec3 Tint;
out vec3 Normal;

uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * vec4(aPos, 1.0);
    TexCoord = aTexCoord;
    Tint = aTint;
    Normal = aNormal;
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 330 core
in vec2 TexCoord;
in vec3 Tint;
in vec3 Normal;

out vec4 color;

uniform sampler2D blockTexture;
uniform vec3 sunDirection;
uniform float ambientLight;
uniform float sunIntensity;

void main() {
    // Sample the texture
    vec4 texColor = texture(blockTexture, TexCoord);
    
    // Apply tint to texture color
    vec3 tintedColor = texColor.rgb * Tint;
    
    // Normalize the normal and sun direction
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(sunDirection);
    
    // Calculate diffuse lighting with sun direction (only when sun is visible)
    float diff = max(dot(norm, lightDir), 0.0) * sunIntensity;
    
    // Add ambient light so we can still see at night
    float totalLight = ambientLight + diff * (1.0 - ambientLight);
    
    // Apply lighting to tinted texture color
    vec3 result = tintedColor * totalLight;
    
    color = vec4(result, texColor.a);
}
"#;
