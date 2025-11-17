use crate::block::BlockType;
use crate::chunk::{CHUNK_HEIGHT, CHUNK_SIZE, Chunk};
use crate::rng::SeededRng;
use crate::terrain::TerrainGenerator;
use std::collections::HashSet;

pub enum TreeType {
    Oak,
    Spruce,
    Birch,
    Cactus,
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
        let local_x = world_x.rem_euclid(CHUNK_SIZE);
        let local_z = world_z.rem_euclid(CHUNK_SIZE);

        if local_x < 3 || local_x >= CHUNK_SIZE - 3 || local_z < 3 || local_z >= CHUNK_SIZE - 3 {
            return false;
        }

        !self.tree_positions.iter().any(|&(tx, tz)| {
            let dx = (tx - world_x).abs();
            let dz = (tz - world_z).abs();
            dx < 6 && dz < 6
        })
    }

    pub fn register_tree(&mut self, world_x: i32, world_z: i32) {
        self.tree_positions.insert((world_x, world_z));
    }

    pub fn spawn_trees_for_chunk(&mut self, chunk: &mut Chunk, terrain_gen: &TerrainGenerator) {
        let water_level = terrain_gen.get_water_level();

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = chunk.pos.x * CHUNK_SIZE + x;
                let world_z = chunk.pos.z * CHUNK_SIZE + z;

                let mut actual_surface = -1;

                for y in (0..CHUNK_HEIGHT).rev() {
                    if chunk.get_block(x, y, z) != BlockType::Air {
                        actual_surface = y;
                        break;
                    }
                }

                if actual_surface >= water_level && actual_surface < CHUNK_HEIGHT - 8 {
                    let biome = terrain_gen.get_biome_at(world_x, actual_surface, world_z);
                    let block = chunk.get_block(x, actual_surface, z);

                    let (should_spawn, surface_requirement, tree_threshold, tree_type) = match biome
                        .name
                        .as_str()
                    {
                        "taiga" => (true, BlockType::Podzol, 0.96, TreeType::Spruce),
                        "snowy_taiga" => (true, BlockType::SnowyGrassBlock, 0.96, TreeType::Spruce),
                        "oak_forest" => (true, BlockType::GrassBlock, 0.94, TreeType::Oak),
                        "birch_forest" => (true, BlockType::GrassBlock, 0.94, TreeType::Birch),
                        "desert" => (true, BlockType::Sand, 0.99, TreeType::Cactus),
                        _ => (false, BlockType::Air, 0.98, TreeType::Oak),
                    };

                    if !should_spawn || block != surface_requirement {
                        continue;
                    }

                    let rng = SeededRng::new(world_x, world_z);
                    let tree_check = rng.next_f32();

                    if tree_check > tree_threshold && self.can_place_tree(world_x, world_z) {
                        let seed = rng.next_u32();

                        if generate_tree(chunk, x, actual_surface + 1, z, seed, tree_type) {
                            self.register_tree(world_x, world_z);
                        }
                    }
                }
            }
        }
    }
}

pub fn generate_tree(
    chunk: &mut Chunk,
    x: i32,
    base_y: i32,
    z: i32,
    seed: u32,
    tree_type: TreeType,
) -> bool {
    match tree_type {
        TreeType::Oak => generate_oak_tree(chunk, x, base_y, z, seed),
        TreeType::Spruce => generate_spruce_tree(chunk, x, base_y, z, seed),
        TreeType::Birch => generate_birch_tree(chunk, x, base_y, z, seed),
        TreeType::Cactus => generate_cactus(chunk, x, base_y, z, seed),
    }
}

fn generate_oak_tree(chunk: &mut Chunk, x: i32, base_y: i32, z: i32, seed: u32) -> bool {
    let rng = SeededRng::from_seed(seed);
    let trunk_height = 4 + ((rng.next_u32() % 2) as i32);

    for i in 0..trunk_height {
        let check_y = base_y + i;
        let current_block = chunk.get_block(x, check_y, z);
        if current_block != BlockType::Air && !current_block.is_transparent() {
            return false;
        }
    }

    let mut leaves_buffer = vec![vec![vec![0u8; 5]; 4]; 5];
    let d = 1;
    for lz in 0..=2 {
        for ly in 0..=2 {
            for lx in 0..=2 {
                leaves_buffer[lx + 1][ly + 1][lz + 1] = 1;
            }
        }
    }

    for i in 0..7 {
        let px = ((rng.next_u32().wrapping_add(i * 3)) % 3) as usize;
        let py = ((rng.next_u32().wrapping_add(i * 5)) % 3) as usize;
        let pz = ((rng.next_u32().wrapping_add(i * 7)) % 3) as usize;

        for dz in 0..=d {
            for dy in 0..=d {
                for dx in 0..=d {
                    let lx = px + dx;
                    let ly = py + dy;
                    let lz = pz + dz;
                    if lx < 5 && ly < 4 && lz < 5 {
                        leaves_buffer[lx][ly][lz] = 1;
                    }
                }
            }
        }
    }

    let p_y = base_y + trunk_height - 1;
    for lz in 0..5 {
        for ly in 0..4 {
            for lx in 0..5 {
                if leaves_buffer[lx][ly][lz] == 1 {
                    let world_x = x + lx as i32 - 2;
                    let world_y = p_y + ly as i32 - 1;
                    let world_z = z + lz as i32 - 2;

                    let current_block = chunk.get_block(world_x, world_y, world_z);
                    if current_block != BlockType::Air && !current_block.is_transparent() {
                        return false;
                    }
                }
            }
        }
    }

    let mut p_y = base_y;
    for _ in 0..trunk_height {
        chunk.set_block(x, p_y, z, BlockType::OakLog);
        p_y += 1;
    }

    p_y -= 1;

    for lz in 0..5 {
        for ly in 0..4 {
            for lx in 0..5 {
                if leaves_buffer[lx][ly][lz] == 1 {
                    let world_x = x + lx as i32 - 2;
                    let world_y = p_y + ly as i32 - 1;
                    let world_z = z + lz as i32 - 2;

                    let current_block = chunk.get_block(world_x, world_y, world_z);
                    if current_block == BlockType::Air || current_block == BlockType::SnowLayer {
                        chunk.set_block(world_x, world_y, world_z, BlockType::OakLeaves);
                    }
                }
            }
        }
    }

    true
}

fn generate_birch_tree(chunk: &mut Chunk, x: i32, base_y: i32, z: i32, seed: u32) -> bool {
    let rng = SeededRng::from_seed(seed);
    let trunk_height = 4 + ((rng.next_u32() % 2) as i32);

    for i in 0..trunk_height {
        let check_y = base_y + i;
        let current_block = chunk.get_block(x, check_y, z);
        if current_block != BlockType::Air && !current_block.is_transparent() {
            return false;
        }
    }

    let mut leaves_buffer = vec![vec![vec![0u8; 5]; 4]; 5];
    let d = 1;
    for lz in 0..=2 {
        for ly in 0..=2 {
            for lx in 0..=2 {
                leaves_buffer[lx + 1][ly + 1][lz + 1] = 1;
            }
        }
    }

    for i in 0..7 {
        let px = ((rng.next_u32().wrapping_add(i * 3)) % 3) as usize;
        let py = ((rng.next_u32().wrapping_add(i * 5)) % 3) as usize;
        let pz = ((rng.next_u32().wrapping_add(i * 7)) % 3) as usize;

        for dz in 0..=d {
            for dy in 0..=d {
                for dx in 0..=d {
                    let lx = px + dx;
                    let ly = py + dy;
                    let lz = pz + dz;
                    if lx < 5 && ly < 4 && lz < 5 {
                        leaves_buffer[lx][ly][lz] = 1;
                    }
                }
            }
        }
    }

    let p_y = base_y + trunk_height - 1;
    for lz in 0..5 {
        for ly in 0..4 {
            for lx in 0..5 {
                if leaves_buffer[lx][ly][lz] == 1 {
                    let world_x = x + lx as i32 - 2;
                    let world_y = p_y + ly as i32 - 1;
                    let world_z = z + lz as i32 - 2;

                    let current_block = chunk.get_block(world_x, world_y, world_z);
                    if current_block != BlockType::Air && !current_block.is_transparent() {
                        return false;
                    }
                }
            }
        }
    }

    let mut p_y = base_y;
    for _ in 0..trunk_height {
        chunk.set_block(x, p_y, z, BlockType::BirchLog);
        p_y += 1;
    }

    p_y -= 1;

    for lz in 0..5 {
        for ly in 0..4 {
            for lx in 0..5 {
                if leaves_buffer[lx][ly][lz] == 1 {
                    let world_x = x + lx as i32 - 2;
                    let world_y = p_y + ly as i32 - 1;
                    let world_z = z + lz as i32 - 2;

                    let current_block = chunk.get_block(world_x, world_y, world_z);
                    if current_block == BlockType::Air || current_block == BlockType::SnowLayer {
                        chunk.set_block(world_x, world_y, world_z, BlockType::BirchLeaves);
                    }
                }
            }
        }
    }

    true
}

fn generate_spruce_tree(chunk: &mut Chunk, x: i32, base_y: i32, z: i32, seed: u32) -> bool {
    let rng = SeededRng::from_seed(seed);
    let trunk_height = 9 + ((rng.next_u32() % 5) as i32);

    for i in 0..trunk_height {
        let check_y = base_y + i;
        let current_block = chunk.get_block(x, check_y, z);
        if current_block != BlockType::Air && !current_block.is_transparent() {
            return false;
        }
    }

    let mut leaves_buffer = vec![vec![vec![0u8; 7]; 10]; 7];
    let mut dev = 3;

    for yy in 0..=2 {
        for zz in 0..=6 {
            for xx in 0..=6 {
                let dist_x = (xx as i32 - 3).abs();
                let dist_z = (zz as i32 - 3).abs();
                if dist_x <= dev && dist_z <= dev {
                    let rng_var = rng.variant((xx + zz * 7 + yy * 49) as u32);
                    let rand_val = (rng_var.next_u32() % 20) as i32;
                    if rand_val <= 19 - dev {
                        leaves_buffer[xx][yy][zz] = 1;
                        leaves_buffer[xx][yy + 1][zz] = 2;
                    }
                }
            }
        }
        dev -= 1;
    }

    leaves_buffer[3][1][3] = 1;
    leaves_buffer[3][2][3] = 1;
    leaves_buffer[3][3][3] = 2;

    let mut my = 0;
    for iii in 0..20 {
        let xi = (rng.variant(iii * 11).next_u32() % 4) as usize;
        let yy = 4 + (rng.variant(iii * 13).next_u32() % 2) as usize;
        let zi = (rng.variant(iii * 17).next_u32() % 4) as usize;
        if yy > my {
            my = yy;
        }
        for zz in zi..=(zi + 1).min(6) {
            for xx in xi..=(xi + 1).min(6) {
                if yy < 10 && zz < 7 && xx < 7 {
                    leaves_buffer[xx][yy][zz] = 1;
                    if yy + 1 < 10 && leaves_buffer[xx][yy + 1][zz] == 0 {
                        leaves_buffer[xx][yy + 1][zz] = 2;
                    }
                }
            }
        }
    }

    dev = 2;
    for yy in (my + 1)..=(my + 2).min(9) {
        for zz in 0..=6 {
            for xx in 0..=6 {
                let dist_x = (xx as i32 - 3).abs();
                let dist_z = (zz as i32 - 3).abs();
                if dist_x <= dev && dist_z <= dev {
                    let rng_var = rng.variant((xx + zz * 7 + yy * 49) as u32);
                    let rand_val = (rng_var.next_u32() % 20) as i32;
                    if rand_val <= 19 - dev {
                        leaves_buffer[xx][yy][zz] = 1;
                        if yy + 1 < 10 {
                            leaves_buffer[xx][yy + 1][zz] = 2;
                        }
                    }
                }
            }
        }
        dev -= 1;
    }

    let p_y = base_y + trunk_height - 1;
    for lz in 0..7 {
        for ly in 0..10 {
            for lx in 0..7 {
                if leaves_buffer[lx][ly][lz] != 0 {
                    let world_x = x + lx as i32 - 3;
                    let world_y = p_y + ly as i32 - 6;
                    let world_z = z + lz as i32 - 3;

                    let current_block = chunk.get_block(world_x, world_y, world_z);
                    if current_block != BlockType::Air && !current_block.is_transparent() {
                        return false;
                    }
                }
            }
        }
    }

    let mut p_y = base_y;
    for _ in 0..trunk_height {
        chunk.set_block(x, p_y, z, BlockType::SpruceLog);
        p_y += 1;
    }

    p_y -= 1;

    for lz in 0..7 {
        for ly in 0..10 {
            for lx in 0..7 {
                if leaves_buffer[lx][ly][lz] == 1 {
                    let world_x = x + lx as i32 - 3;
                    let world_y = p_y + ly as i32 - 6;
                    let world_z = z + lz as i32 - 3;

                    let current_block = chunk.get_block(world_x, world_y, world_z);
                    if current_block == BlockType::Air || current_block == BlockType::SnowLayer {
                        chunk.set_block(world_x, world_y, world_z, BlockType::SpruceLeaves);
                    }
                }
            }
        }
    }

    true
}

fn generate_cactus(chunk: &mut Chunk, x: i32, base_y: i32, z: i32, seed: u32) -> bool {
    let rng = SeededRng::from_seed(seed);
    let trunk_height = 2 + ((rng.next_u32() % 3) as i32);

    for i in 0..trunk_height {
        let check_y = base_y + i;
        let current_block = chunk.get_block(x, check_y, z);
        if current_block != BlockType::Air && !current_block.is_transparent() {
            return false;
        }
    }

    let mut p_y = base_y;
    for _ in 0..trunk_height {
        chunk.set_block(x, p_y, z, BlockType::Cactus);
        p_y += 1;
    }

    let flower_check = rng.next_f32();
    if flower_check < 0.3 {
        let flower_y = base_y + trunk_height;
        if flower_y < CHUNK_HEIGHT {
            let current_block = chunk.get_block(x, flower_y, z);
            if current_block == BlockType::Air || current_block.is_transparent() {
                chunk.set_block(x, flower_y, z, BlockType::CactusFlower);
            }
        }
    }

    true
}
