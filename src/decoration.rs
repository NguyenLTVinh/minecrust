use crate::block::BlockType;
use crate::chunk::{CHUNK_HEIGHT, CHUNK_SIZE, Chunk};
use crate::texture::TextureCoords;
use std::collections::HashSet;

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
}

pub fn generate_tree(chunk: &mut Chunk, x: i32, base_y: i32, z: i32, trunk_height: i32) {
    let leaf_radius = 2;

    for y in base_y..(base_y + trunk_height) {
        chunk.set_block(x, y, z, BlockType::Wood);
    }

    let leaf_start = base_y + trunk_height - 2;
    for dy in 0..4 {
        let y = leaf_start + dy;
        let radius = if dy == 3 { 1 } else { leaf_radius };

        for dx in -radius..=radius {
            for dz in -radius..=radius {
                let dist = (dx * dx + dz * dz) as f32;
                if dist <= (radius * radius) as f32 {
                    let block = chunk.get_block(x + dx, y, z + dz);
                    if block == BlockType::Air {
                        chunk.set_block(x + dx, y, z + dz, BlockType::Leaves);
                    }
                }
            }
        }
    }
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
                        BlockType::Leaves => {
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
