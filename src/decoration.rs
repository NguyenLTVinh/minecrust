use crate::block::BlockType;
use crate::chunk::{CHUNK_HEIGHT, CHUNK_SIZE, Chunk};
use crate::texture::TextureCoords;
use std::collections::HashSet;

pub struct TreeGenerator {
    tree_positions: HashSet<(i32, i32)>,
}

pub struct DecorationGenerator;

impl DecorationGenerator {
    pub fn new() -> Self {
        DecorationGenerator
    }

    pub fn generate_decorations(chunk: &mut Chunk, biome_name: &str) {
        let world_x_base = chunk.pos.x * CHUNK_SIZE;
        let world_z_base = chunk.pos.z * CHUNK_SIZE;

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = world_x_base + x;
                let world_z = world_z_base + z;

                for y in (0..CHUNK_HEIGHT).rev() {
                    let block = chunk.get_block(x, y, z);

                    if block == BlockType::Air || block == BlockType::Water {
                        continue;
                    }

                    let surface_y = y + 1;
                    if surface_y < CHUNK_HEIGHT
                        && chunk.get_block(x, surface_y, z) == BlockType::Air
                    {
                        match biome_name {
                            "grassland" => {
                                if block == BlockType::Grass {
                                    Self::generate_grassland_decoration(
                                        chunk, x, surface_y, z, world_x, world_z,
                                    );
                                }
                            }
                            "forest" => {
                                if block == BlockType::Grass {
                                    Self::generate_forest_decoration(
                                        chunk, x, surface_y, z, world_x, world_z,
                                    );
                                }
                            }
                            "desert" => {
                                if block == BlockType::Sand {
                                    Self::generate_desert_decoration(
                                        chunk, x, surface_y, z, world_x, world_z,
                                    );
                                }
                            }
                            _ => {}
                        }
                    }

                    break;
                }
            }
        }
    }

    fn generate_grassland_decoration(
        chunk: &mut Chunk,
        x: i32,
        surface_y: i32,
        z: i32,
        world_x: i32,
        world_z: i32,
    ) {
        let noise_seed = ((world_x as u32)
            .wrapping_mul(374761393)
            .wrapping_add((world_z as u32).wrapping_mul(668265263)))
            % 1000;
        let noise_value = noise_seed as f32 / 1000.0;

        if noise_value > 0.96 {
            let plant_type_seed = ((world_x as u32)
                .wrapping_mul(109739919)
                .wrapping_add((world_z as u32).wrapping_mul(715827883)))
                % 100;

            let plant = if plant_type_seed < 25 {
                BlockType::ShortGrass
            } else if plant_type_seed < 45 {
                // Tall grass: bottom + top
                chunk.set_block(x, surface_y, z, BlockType::TallGrassBottom);
                if surface_y + 1 < CHUNK_HEIGHT {
                    chunk.set_block(x, surface_y + 1, z, BlockType::TallGrassTop);
                }
                return;
            } else if plant_type_seed < 65 {
                BlockType::Poppy
            } else if plant_type_seed < 80 {
                BlockType::PinkTulip
            } else {
                BlockType::TorchFlower
            };

            chunk.set_block(x, surface_y, z, plant);
        }
    }

    fn generate_forest_decoration(
        chunk: &mut Chunk,
        x: i32,
        surface_y: i32,
        z: i32,
        world_x: i32,
        world_z: i32,
    ) {
        let noise_seed = ((world_x as u32)
            .wrapping_mul(374761393)
            .wrapping_add((world_z as u32).wrapping_mul(668265263)))
            % 1000;
        let noise_value = noise_seed as f32 / 1000.0;

        // Forests have very sparse decorations
        if noise_value > 0.97 {
            let plant_type_seed = ((world_x as u32)
                .wrapping_mul(109739919)
                .wrapping_add((world_z as u32).wrapping_mul(715827883)))
                % 100;

            let plant = if plant_type_seed < 50 {
                BlockType::RedMushroom
            } else {
                BlockType::BrownMushroom
            };

            chunk.set_block(x, surface_y, z, plant);
        }
    }

    fn generate_desert_decoration(
        chunk: &mut Chunk,
        x: i32,
        surface_y: i32,
        z: i32,
        world_x: i32,
        world_z: i32,
    ) {
        let noise_seed = ((world_x as u32)
            .wrapping_mul(374761393)
            .wrapping_add((world_z as u32).wrapping_mul(668265263)))
            % 1000;
        let noise_value = noise_seed as f32 / 1000.0;

        // Deserts are mostly empty
        if noise_value > 0.99 {
            let plant_type_seed = ((world_x as u32)
                .wrapping_mul(109739919)
                .wrapping_add((world_z as u32).wrapping_mul(715827883)))
                % 100;

            let plant = if plant_type_seed < 35 {
                BlockType::ShortDryGrass
            } else if plant_type_seed < 65 {
                BlockType::TallDryGrass
            } else {
                BlockType::DeadBush
            };

            chunk.set_block(x, surface_y, z, plant);
        }
    }
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
}

pub enum TreeType {
    Oak,
    Spruce,
    Birch,
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
    }
}

fn generate_oak_tree(chunk: &mut Chunk, x: i32, base_y: i32, z: i32, seed: u32) -> bool {
    let trunk_height = 4 + ((seed % 2) as i32);

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

    let rng_val = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    for i in 0..7 {
        let px = ((rng_val.wrapping_add(i * 3)) % 3) as usize;
        let py = ((rng_val.wrapping_add(i * 5)) % 3) as usize;
        let pz = ((rng_val.wrapping_add(i * 7)) % 3) as usize;

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
    let trunk_height = 4 + ((seed % 2) as i32);

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

    let rng_val = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    for i in 0..7 {
        let px = ((rng_val.wrapping_add(i * 3)) % 3) as usize;
        let py = ((rng_val.wrapping_add(i * 5)) % 3) as usize;
        let pz = ((rng_val.wrapping_add(i * 7)) % 3) as usize;

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
    let trunk_height = 9 + ((seed % 5) as i32);

    for i in 0..trunk_height {
        let check_y = base_y + i;
        let current_block = chunk.get_block(x, check_y, z);
        if current_block != BlockType::Air && !current_block.is_transparent() {
            return false;
        }
    }

    let mut leaves_buffer = vec![vec![vec![0u8; 7]; 10]; 7];
    let mut dev = 3;
    let rng = seed.wrapping_mul(1664525).wrapping_add(1013904223);

    for yy in 0..=2 {
        for zz in 0..=6 {
            for xx in 0..=6 {
                let dist_x = (xx as i32 - 3).abs();
                let dist_z = (zz as i32 - 3).abs();
                if dist_x <= dev && dist_z <= dev {
                    let rand_val = ((rng.wrapping_add((xx + zz * 7 + yy * 49) as u32)) % 20) as i32;
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
        let xi = ((rng.wrapping_add(iii * 11)) % 4) as usize;
        let yy = 4 + ((rng.wrapping_add(iii * 13)) % 2) as usize;
        let zi = ((rng.wrapping_add(iii * 17)) % 4) as usize;
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
                    let rand_val = ((rng.wrapping_add((xx + zz * 7 + yy * 49) as u32)) % 20) as i32;
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
                } else if leaves_buffer[lx][ly][lz] == 2 {
                    let world_x = x + lx as i32 - 3;
                    let world_y = p_y + ly as i32 - 6;
                    let world_z = z + lz as i32 - 3;

                    let current_block = chunk.get_block(world_x, world_y, world_z);
                    if current_block == BlockType::Air || current_block == BlockType::SnowLayer {
                        chunk.set_block(world_x, world_y, world_z, BlockType::SnowLayer);
                    }
                }
            }
        }
    }

    true
}

pub fn generate_snow(chunk: &mut Chunk, snow_start_altitude: i32, snow_full_altitude: i32) {
    let world_x_base = chunk.pos.x * CHUNK_SIZE;
    let world_z_base = chunk.pos.z * CHUNK_SIZE;

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let world_x = world_x_base + x;
            let world_z = world_z_base + z;

            for y in (0..CHUNK_HEIGHT).rev() {
                let block = chunk.get_block(x, y, z);

                if block == BlockType::Air || block == BlockType::Water {
                    continue;
                }

                let snow_factor = if y >= snow_full_altitude {
                    1.0
                } else if y >= snow_start_altitude {
                    (y - snow_start_altitude) as f32
                        / (snow_full_altitude - snow_start_altitude) as f32
                } else {
                    0.0
                };

                let variation_seed = ((world_x * 2345 + world_z * 5432 + y * 3456) as u32) % 1000;
                let blend_noise = (variation_seed as f32) / 1000.0;

                let adjusted_factor = snow_factor + (blend_noise - 0.5) * 0.3;

                if adjusted_factor > 0.5 {
                    match block {
                        BlockType::Grass => {
                            let replace_seed = ((world_x * 9876 + world_z * 5432) as u32) % 100;

                            if replace_seed < 30 && adjusted_factor > 0.7 {
                                chunk.set_block(x, y, z, BlockType::Snow);
                            } else {
                                chunk.set_block(x, y, z, BlockType::GrassSnowy);
                            }
                            let snow_y = y + 1;
                            if snow_y < CHUNK_HEIGHT
                                && chunk.get_block(x, snow_y, z) == BlockType::Air
                            {
                                chunk.set_block(x, snow_y, z, BlockType::SnowLayer);
                            }
                        }
                        BlockType::OakLeaves => {
                            if adjusted_factor > 0.7 {
                                let snow_y = y + 1;
                                if snow_y < CHUNK_HEIGHT
                                    && chunk.get_block(x, snow_y, z) == BlockType::Air
                                {
                                    chunk.set_block(x, snow_y, z, BlockType::SnowLayer);
                                }
                            }
                        }
                        BlockType::Stone | BlockType::Dirt => {
                            let snow_y = y;
                            if snow_y < CHUNK_HEIGHT
                                && chunk.get_block(x, snow_y, z) == BlockType::Air
                            {
                                chunk.set_block(x, snow_y, z, BlockType::SnowLayer);
                            }
                        }
                        _ => {}
                    }
                }

                break;
            }
        }
    }
}

pub fn add_snow_layer(
    vertices: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
    height: f32,
    tex: TextureCoords,
    tint: [f32; 3],
) {
    let top_normal = [0.0, 1.0, 0.0];
    #[rustfmt::skip]
    let top_verts = vec![
        x,         y + height, z,
        x,         y + height, z + 1.0,
        x + 1.0,   y + height, z + 1.0,
        
        x,         y + height, z,
        x + 1.0,   y + height, z + 1.0,
        x + 1.0,   y + height, z,
    ];
    let top_uvs = [
        [tex.u_min, tex.v_min],
        [tex.u_min, tex.v_max],
        [tex.u_max, tex.v_max],
        [tex.u_min, tex.v_min],
        [tex.u_max, tex.v_max],
        [tex.u_max, tex.v_min],
    ];
    for (i, pos_idx) in (0..top_verts.len()).step_by(3).enumerate() {
        vertices.extend_from_slice(&top_verts[pos_idx..pos_idx + 3]);
        vertices.extend_from_slice(&top_uvs[i]);
        vertices.extend_from_slice(&tint);
        vertices.extend_from_slice(&top_normal);
    }

    let bottom_normal = [0.0, -1.0, 0.0];

    #[rustfmt::skip]
    let bottom_verts = vec![
        x,         y, z,
        x + 1.0,   y, z,
        x + 1.0,   y, z + 1.0,
        
        x,         y, z,
        x + 1.0,   y, z + 1.0,
        x,         y, z + 1.0,
    ];
    let bottom_uvs = [
        [tex.u_min, tex.v_min],
        [tex.u_min, tex.v_max],
        [tex.u_max, tex.v_max],
        [tex.u_min, tex.v_min],
        [tex.u_max, tex.v_max],
        [tex.u_max, tex.v_min],
    ];
    for (i, pos_idx) in (0..bottom_verts.len()).step_by(3).enumerate() {
        vertices.extend_from_slice(&bottom_verts[pos_idx..pos_idx + 3]);
        vertices.extend_from_slice(&bottom_uvs[i]);
        vertices.extend_from_slice(&tint);
        vertices.extend_from_slice(&bottom_normal);
    }

    let side_v_max = tex.v_min + (tex.v_max - tex.v_min) * height;

    let front_normal = [0.0, 0.0, 1.0];

    #[rustfmt::skip]
    let front_verts = vec![
        x,         y,          z + 1.0,
        x,         y + height, z + 1.0,
        x + 1.0,   y + height, z + 1.0,
        
        x,         y,          z + 1.0,
        x + 1.0,   y + height, z + 1.0,
        x + 1.0,   y,          z + 1.0,
    ];
    let front_uvs = [
        [tex.u_min, side_v_max],
        [tex.u_min, tex.v_min],
        [tex.u_max, tex.v_min],
        [tex.u_min, side_v_max],
        [tex.u_max, tex.v_min],
        [tex.u_max, side_v_max],
    ];
    for (i, pos_idx) in (0..front_verts.len()).step_by(3).enumerate() {
        vertices.extend_from_slice(&front_verts[pos_idx..pos_idx + 3]);
        vertices.extend_from_slice(&front_uvs[i]);
        vertices.extend_from_slice(&tint);
        vertices.extend_from_slice(&front_normal);
    }

    let back_normal = [0.0, 0.0, -1.0];
    #[rustfmt::skip]
    let back_verts = vec![
        x,         y,          z,
        x + 1.0,   y,          z,
        x + 1.0,   y + height, z,
        
        x,         y,          z,
        x + 1.0,   y + height, z,
        x,         y + height, z,
    ];
    let back_uvs = [
        [tex.u_min, side_v_max],
        [tex.u_max, side_v_max],
        [tex.u_max, tex.v_min],
        [tex.u_min, side_v_max],
        [tex.u_max, tex.v_min],
        [tex.u_min, tex.v_min],
    ];
    for (i, pos_idx) in (0..back_verts.len()).step_by(3).enumerate() {
        vertices.extend_from_slice(&back_verts[pos_idx..pos_idx + 3]);
        vertices.extend_from_slice(&back_uvs[i]);
        vertices.extend_from_slice(&tint);
        vertices.extend_from_slice(&back_normal);
    }

    let right_normal = [1.0, 0.0, 0.0];
    #[rustfmt::skip]
    let right_verts = vec![
        x + 1.0,   y,          z,
        x + 1.0,   y + height, z,
        x + 1.0,   y + height, z + 1.0,
        
        x + 1.0,   y,          z,
        x + 1.0,   y + height, z + 1.0,
        x + 1.0,   y,          z + 1.0,
    ];
    let right_uvs = [
        [tex.u_min, side_v_max],
        [tex.u_min, tex.v_min],
        [tex.u_max, tex.v_min],
        [tex.u_min, side_v_max],
        [tex.u_max, tex.v_min],
        [tex.u_max, side_v_max],
    ];
    for (i, pos_idx) in (0..right_verts.len()).step_by(3).enumerate() {
        vertices.extend_from_slice(&right_verts[pos_idx..pos_idx + 3]);
        vertices.extend_from_slice(&right_uvs[i]);
        vertices.extend_from_slice(&tint);
        vertices.extend_from_slice(&right_normal);
    }

    let left_normal = [-1.0, 0.0, 0.0];
    #[rustfmt::skip]
    let left_verts = vec![
        x, y,          z,
        x, y,          z + 1.0,
        x, y + height, z + 1.0,
        
        x, y,          z,
        x, y + height, z + 1.0,
        x, y + height, z,
    ];
    let left_uvs = [
        [tex.u_max, side_v_max],
        [tex.u_min, side_v_max],
        [tex.u_min, tex.v_min],
        [tex.u_max, side_v_max],
        [tex.u_min, tex.v_min],
        [tex.u_max, tex.v_min],
    ];
    for (i, pos_idx) in (0..left_verts.len()).step_by(3).enumerate() {
        vertices.extend_from_slice(&left_verts[pos_idx..pos_idx + 3]);
        vertices.extend_from_slice(&left_uvs[i]);
        vertices.extend_from_slice(&tint);
        vertices.extend_from_slice(&left_normal);
    }
}
