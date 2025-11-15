use crate::biome;
use crate::block::{BlockType, FaceDirection};
use crate::decoration::DecorationGenerator;
use crate::mesh_builder::MeshBuilder;
use crate::terrain::TerrainGenerator;
use crate::texture::TextureAtlas;
use crate::tree_generator::TreeGenerator;
use gl::types::*;
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

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: BlockType) {
        if x < 0 || x >= CHUNK_SIZE || y < 0 || y >= CHUNK_HEIGHT || z < 0 || z >= CHUNK_SIZE {
            return;
        }
        self.blocks[Self::get_index(x, y, z)] = block;
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

pub fn generate_chunk_mesh(chunk: &Chunk, atlas: &TextureAtlas) -> Vec<f32> {
    let mut vertices = Vec::new();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                let block = chunk.get_block(x, y, z);
                if block == BlockType::Air {
                    continue;
                }

                let wx = (chunk.pos.x * CHUNK_SIZE + x) as f32;
                let wy = y as f32;
                let wz = (chunk.pos.z * CHUNK_SIZE + z) as f32;

                if block == BlockType::SnowLayer {
                    let height = 0.125;
                    let tex_coords = atlas.get_tex_coords(block, FaceDirection::Top);
                    let tint = atlas.get_tint(block);
                    MeshBuilder::add_snow_layer(
                        &mut vertices,
                        wx,
                        wy,
                        wz,
                        height,
                        tex_coords,
                        tint,
                    );
                    continue;
                }

                if block.is_decorative() {
                    let tex_coords = atlas.get_tex_coords(block, FaceDirection::Top);
                    let tint = atlas.get_tint(block);
                    MeshBuilder::add_cross_plant(&mut vertices, wx, wy, wz, tex_coords, tint);
                    continue;
                }

                let faces = [
                    (FaceDirection::Top, 0, 1, 0),
                    (FaceDirection::Bottom, 0, -1, 0),
                    (FaceDirection::Front, 0, 0, 1),
                    (FaceDirection::Back, 0, 0, -1),
                    (FaceDirection::Right, 1, 0, 0),
                    (FaceDirection::Left, -1, 0, 0),
                ];

                for (face_dir, dx, dy, dz) in faces {
                    let adjacent = chunk.get_block(x + dx, y + dy, z + dz);

                    let should_render = if block == BlockType::Water {
                        adjacent == BlockType::Air
                    } else {
                        adjacent.is_transparent()
                    };

                    if should_render {
                        let tex_coords = atlas.get_tex_coords(block, face_dir);
                        let tint = match block {
                            BlockType::Grass | BlockType::GrassSnowy => {
                                if face_dir == FaceDirection::Top {
                                    atlas.get_tint(block)
                                } else {
                                    [1.0, 1.0, 1.0]
                                }
                            }
                            _ => atlas.get_tint(block),
                        };
                        MeshBuilder::add_face(
                            &mut vertices,
                            wx,
                            wy,
                            wz,
                            dx,
                            dy,
                            dz,
                            tex_coords,
                            tint,
                        );
                    }
                }
            }
        }
    }

    vertices
}
