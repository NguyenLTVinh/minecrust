use crate::block::BlockType;
use crate::chunk::{CHUNK_HEIGHT, CHUNK_SIZE, Chunk};
use crate::rng::SeededRng;
use crate::terrain::TerrainGenerator;

#[derive(Clone, Debug)]
pub struct Biome {
    pub name: String,
    pub c_top: BlockType,
    pub c_filler: BlockType,
    pub c_stone: BlockType,
    pub c_water: BlockType,
    pub c_river_water: BlockType,
    pub depth_top: i32,
    pub depth_filler: i32,
    pub heat_point: f32,
    pub humidity_point: f32,
    pub min_y: i32,
    pub max_y: i32,
    pub vertical_blend: i32,
    pub weight: f32,
    pub surface_variant: Option<BlockType>,
    pub surface_variant_frequency: f32,
}

impl Biome {
    pub fn new(
        name: &str,
        c_top: BlockType,
        c_filler: BlockType,
        c_stone: BlockType,
        c_water: BlockType,
        c_river_water: BlockType,
        depth_top: i32,
        depth_filler: i32,
        heat_point: f32,
        humidity_point: f32,
        min_y: i32,
        max_y: i32,
        vertical_blend: i32,
    ) -> Self {
        Biome {
            name: name.to_string(),
            c_top,
            c_filler,
            c_stone,
            c_water,
            c_river_water,
            depth_top,
            depth_filler,
            heat_point,
            humidity_point,
            min_y,
            max_y,
            vertical_blend,
            weight: 1.0,
            surface_variant: None,
            surface_variant_frequency: 0.0,
        }
    }

    pub fn with_surface_variant(mut self, variant: BlockType, frequency: f32) -> Self {
        self.surface_variant = Some(variant);
        self.surface_variant_frequency = frequency;
        self
    }
}

pub struct BiomeManager {
    biomes: Vec<Biome>,
}

impl BiomeManager {
    pub fn new() -> Self {
        let mut manager = BiomeManager { biomes: Vec::new() };

        manager.register_default_biomes();
        manager
    }

    fn register_default_biomes(&mut self) {
        self.register(Biome::new(
            "grassland",
            BlockType::Grass,
            BlockType::Dirt,
            BlockType::Stone,
            BlockType::Water,
            BlockType::Water,
            1,
            2,
            50.0,
            50.0,
            0,
            40,
            16,
        ));

        self.register(Biome::new(
            "desert",
            BlockType::Sand,
            BlockType::Sand,
            BlockType::Stone,
            BlockType::Water,
            BlockType::Water,
            1,
            1,
            60.0,
            10.0,
            0,
            40,
            8,
        ));

        self.register(
            Biome::new(
                "taiga",
                BlockType::Grass,
                BlockType::Dirt,
                BlockType::Stone,
                BlockType::Water,
                BlockType::Water,
                1,
                3,
                20.0,
                70.0,
                0,
                60,
                16,
            )
            .with_surface_variant(BlockType::Podzol, 0.01),
        );

        self.register(Biome::new(
            "snowy_taiga",
            BlockType::GrassSnowy,
            BlockType::Dirt,
            BlockType::Stone,
            BlockType::Water,
            BlockType::Water,
            1,
            3,
            -10.0,
            70.0,
            0,
            255,
            16,
        ));

        self.register(Biome::new(
            "tundra",
            BlockType::Snow,
            BlockType::Snow,
            BlockType::Stone,
            BlockType::Water,
            BlockType::Water,
            1,
            2,
            -20.0,
            40.0,
            0,
            40,
            16,
        ));

        self.register(Biome::new(
            "oak_forest",
            BlockType::Grass,
            BlockType::Dirt,
            BlockType::Stone,
            BlockType::Water,
            BlockType::Water,
            1,
            3,
            40.0,
            75.0,
            0,
            60,
            16,
        ));

        self.register(Biome::new(
            "birch_forest",
            BlockType::Grass,
            BlockType::Dirt,
            BlockType::Stone,
            BlockType::Water,
            BlockType::Water,
            1,
            3,
            35.0,
            80.0,
            0,
            60,
            16,
        ));
    }

    pub fn register(&mut self, biome: Biome) {
        self.biomes.push(biome);
    }

    pub fn get_biome_at_pos(&self, heat: f32, humidity: f32, y: i32) -> &Biome {
        let mut biome_closest: Option<&Biome> = None;
        let mut biome_closest_blend: Option<&Biome> = None;
        let mut dist_min = f32::INFINITY;
        let mut dist_min_blend = f32::INFINITY;

        for biome in &self.biomes {
            if y < biome.min_y || y > biome.max_y + biome.vertical_blend {
                continue;
            }

            let d_heat = heat - biome.heat_point;
            let d_humidity = humidity - biome.humidity_point;
            let mut dist = (d_heat * d_heat) + (d_humidity * d_humidity);

            if biome.weight > 0.0 {
                dist /= biome.weight;
            }

            if y <= biome.max_y {
                if dist < dist_min {
                    dist_min = dist;
                    biome_closest = Some(biome);
                }
            } else if dist < dist_min_blend {
                dist_min_blend = dist;
                biome_closest_blend = Some(biome);
            }
        }

        let seed = ((y as f64) + (heat as f64 + humidity as f64) * 0.9) as u64;
        let rng = SeededRng::from_seed(seed as u32);
        let rng_val = (rng.next_u32() >> 1) as i32;

        if let Some(blend_biome) = biome_closest_blend {
            if dist_min_blend <= dist_min {
                let blend_chance = blend_biome.vertical_blend;
                let blend_range = y - blend_biome.max_y;
                if blend_chance > 0 && (rng_val % blend_chance) >= blend_range {
                    return blend_biome;
                }
            }
        }

        if let Some(closest) = biome_closest {
            return closest;
        }

        &self.biomes[0]
    }
}

pub fn generate_surface_variants(chunk: &mut Chunk, terrain_gen: &TerrainGenerator) {
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let world_x = chunk.pos.x * CHUNK_SIZE + x;
            let world_z = chunk.pos.z * CHUNK_SIZE + z;

            for y in (0..CHUNK_HEIGHT).rev() {
                let block = chunk.get_block(x, y, z);

                if block == BlockType::Air || block == BlockType::Water {
                    continue;
                }

                let biome = terrain_gen.get_biome_at(world_x, y, world_z);

                if let Some(variant) = biome.surface_variant {
                    if block == biome.c_top && biome.surface_variant_frequency > 0.0 {
                        let noise_val = ((world_x as u32)
                            .wrapping_mul(374761393)
                            .wrapping_add((world_z as u32).wrapping_mul(668265263))
                            .wrapping_add(y as u32)
                            .wrapping_mul(715827883))
                            % 1000;
                        let noise_factor = (noise_val as f32) / 1000.0;

                        if noise_factor < biome.surface_variant_frequency {
                            let radius = 10;
                            for px in -radius..=radius {
                                for pz in -radius..=radius {
                                    let dist_sq = px * px + pz * pz;
                                    let max_dist_sq = radius * radius;
                                    let dist_factor = (max_dist_sq as f32 - dist_sq as f32)
                                        / (max_dist_sq as f32);

                                    let patch_noise = ((world_x as u32)
                                        .wrapping_mul(374761393)
                                        .wrapping_add((world_z as u32).wrapping_mul(668265263))
                                        .wrapping_add(px as u32)
                                        .wrapping_mul(109739919)
                                        .wrapping_add(pz as u32)
                                        .wrapping_mul(715827883))
                                        % 1000;
                                    let patch_factor = (patch_noise as f32) / 1000.0;

                                    if dist_factor * 0.6 + patch_factor * 0.4 > 0.35 && dist_sq > 0
                                    {
                                        let px_local = x + px;
                                        let pz_local = z + pz;
                                        for dy in -1..=1 {
                                            let target_y = y + dy;
                                            if target_y >= 0 && target_y < CHUNK_HEIGHT {
                                                let block_at =
                                                    chunk.get_block(px_local, target_y, pz_local);
                                                if matches!(
                                                    block_at,
                                                    BlockType::Grass
                                                        | BlockType::Dirt
                                                        | BlockType::GrassSnowy
                                                ) {
                                                    chunk.set_block(
                                                        px_local, target_y, pz_local, variant,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                break;
            }
        }
    }
}

pub fn generate_snow(
    chunk: &mut Chunk,
    snow_start_altitude: i32,
    snow_full_altitude: i32,
    _biome_name: &str,
) {
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
                        _ if block.is_full_block() => {
                            let snow_y = y + 1;
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
