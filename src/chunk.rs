use crate::biome;
use crate::block::{BlockType, Rotation};
use crate::decoration::DecorationGenerator;
use crate::terrain::TerrainGenerator;
use crate::tree_generator::TreeGenerator;
use gl::types::*;
use std::collections::HashMap;
use std::mem;
use std::ptr;

pub const CHUNK_SIZE: i32 = 16;
pub const CHUNK_HEIGHT: i32 = 160;
pub const RENDER_DISTANCE: i32 = 16;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

pub struct Chunk {
    pub blocks: Vec<BlockType>,
    pub rotations: HashMap<(i32, i32, i32), Rotation>,
    pub mesh: Option<ChunkMesh>,
    pub pos: ChunkPos,
}

impl Chunk {
    pub fn new(
        pos: ChunkPos,
        terrain_gen: &TerrainGenerator,
        tree_generator: &mut TreeGenerator,
    ) -> Self {
        let mut chunk = Chunk {
            blocks: vec![BlockType::Air; (CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE) as usize],
            rotations: HashMap::new(),
            mesh: None,
            pos,
        };

        let water_level = terrain_gen.get_water_level();

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = pos.x * CHUNK_SIZE + x;
                let world_z = pos.z * CHUNK_SIZE + z;

                let _filler_depth = terrain_gen.get_filler_depth(world_x, world_z).max(0);
                let mut depth_count = 0;

                for y in (0..CHUNK_HEIGHT).rev() {
                    let idx = Self::get_index(x, y, z);
                    let biome = terrain_gen.get_biome_at(world_x, y, world_z);

                    if terrain_gen.is_solid_at(world_x, y, world_z) {
                        depth_count += 1;

                        chunk.blocks[idx] = if depth_count == 1 && y >= water_level {
                            biome.c_top
                        } else if depth_count <= biome.depth_top + biome.depth_filler {
                            biome.c_filler
                        } else {
                            biome.c_stone
                        };
                    } else if terrain_gen.is_water_at(world_x, y, world_z) {
                        chunk.blocks[idx] = biome.c_water;
                        depth_count = 0;
                    }
                }
            }
        }

        let test_biome = terrain_gen.get_biome_at(pos.x * CHUNK_SIZE, 32, pos.z * CHUNK_SIZE);
        biome::generate_surface_variants(&mut chunk, terrain_gen);
        tree_generator.spawn_trees_for_chunk(&mut chunk, terrain_gen);
        biome::generate_snow(&mut chunk, 80, 95, &test_biome.name);
        DecorationGenerator::generate_decorations(&mut chunk, &test_biome.name);

        chunk
    }

    fn get_index(x: i32, y: i32, z: i32) -> usize {
        (x + z * CHUNK_SIZE + y * CHUNK_SIZE * CHUNK_SIZE) as usize
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> BlockType {
        if x < 0 || x >= CHUNK_SIZE || y < 0 || y >= CHUNK_HEIGHT || z < 0 || z >= CHUNK_SIZE {
            return BlockType::Air;
        }
        self.blocks[Self::get_index(x, y, z)]
    }

    pub fn get_rotation(&self, x: i32, y: i32, z: i32) -> Rotation {
        if x < 0 || x >= CHUNK_SIZE || y < 0 || y >= CHUNK_HEIGHT || z < 0 || z >= CHUNK_SIZE {
            return Rotation::none();
        }
        self.rotations
            .get(&(x, y, z))
            .copied()
            .unwrap_or(Rotation::none())
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: BlockType, rotation: Rotation) {
        if x < 0 || x >= CHUNK_SIZE || y < 0 || y >= CHUNK_HEIGHT || z < 0 || z >= CHUNK_SIZE {
            return;
        }
        let index = Self::get_index(x, y, z);
        self.blocks[index] = block;

        if block == BlockType::Air {
            self.rotations.remove(&(x, y, z));
        } else if rotation != Rotation::none() {
            self.rotations.insert((x, y, z), rotation);
        } else {
            self.rotations.remove(&(x, y, z));
        }
        self.mesh = None;
    }
}

pub struct ChunkMesh {
    vao: GLuint,
    vbo: GLuint,
    vertex_count: i32,
}

impl ChunkMesh {
    pub fn new(vertices: &[f32]) -> Self {
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

            let stride = 11 * mem::size_of::<f32>() as GLsizei;

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (5 * mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(2);

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

    pub fn render(&self) {
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
