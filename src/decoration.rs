use crate::block::{BlockType, Rotation};
use crate::chunk::{CHUNK_HEIGHT, CHUNK_SIZE, Chunk};
use crate::rng::SeededRng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrequencyTier {
    Frequent,
    Moderate,
    Rare,
}

pub struct DecorationGenerator;

impl DecorationGenerator {
    fn get_frequency_threshold(base_threshold: f32, tier: FrequencyTier) -> f32 {
        match tier {
            FrequencyTier::Frequent => base_threshold,
            FrequencyTier::Moderate => base_threshold + (1.0 - base_threshold) * 0.5,
            FrequencyTier::Rare => base_threshold + (1.0 - base_threshold) * 0.75,
        }
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
                                if block == BlockType::GrassBlock {
                                    Self::generate_grassland_decoration(
                                        chunk, x, surface_y, z, world_x, world_z,
                                    );
                                }
                            }
                            "oak_forest" | "birch_forest" => {
                                if block == BlockType::GrassBlock {
                                    Self::generate_forest_decoration(
                                        chunk, x, surface_y, z, world_x, world_z,
                                    );
                                }
                            }
                            "taiga" => {
                                if block == BlockType::GrassBlock {
                                    Self::generate_taiga_decoration(
                                        chunk, x, surface_y, z, world_x, world_z,
                                    );
                                } else if block == BlockType::Podzol {
                                    Self::generate_podzol_decoration(
                                        chunk, x, surface_y, z, world_x, world_z,
                                    );
                                }
                            }
                            "snowy_taiga" => {}
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
        let rng = SeededRng::new(world_x, world_z);
        let noise_value = rng.noise_f32();
        let plant_type_seed = rng.plant_type_seed();

        if plant_type_seed < 25 {
            let threshold = Self::get_frequency_threshold(0.96, FrequencyTier::Frequent);
            if noise_value > threshold {
                chunk.set_block(x, surface_y, z, BlockType::ShortGrass, Rotation::none());
            }
        } else if plant_type_seed < 45 {
            let threshold = Self::get_frequency_threshold(0.96, FrequencyTier::Moderate);
            if noise_value > threshold {
                chunk.set_block(
                    x,
                    surface_y,
                    z,
                    BlockType::TallGrassBottom,
                    Rotation::none(),
                );
                if surface_y + 1 < CHUNK_HEIGHT {
                    chunk.set_block(
                        x,
                        surface_y + 1,
                        z,
                        BlockType::TallGrassTop,
                        Rotation::none(),
                    );
                }
            }
        } else if plant_type_seed < 65 {
            let threshold = Self::get_frequency_threshold(0.96, FrequencyTier::Moderate);
            if noise_value > threshold {
                chunk.set_block(x, surface_y, z, BlockType::Poppy, Rotation::none());
            }
        } else if plant_type_seed < 80 {
            let threshold = Self::get_frequency_threshold(0.96, FrequencyTier::Moderate);
            if noise_value > threshold {
                chunk.set_block(x, surface_y, z, BlockType::PinkTulip, Rotation::none());
            }
        } else {
            let threshold = Self::get_frequency_threshold(0.96, FrequencyTier::Rare);
            if noise_value > threshold {
                chunk.set_block(x, surface_y, z, BlockType::TorchFlower, Rotation::none());
            }
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
        let rng = SeededRng::new(world_x, world_z);
        let noise_value = rng.noise_f32();
        let plant_type_seed = rng.plant_type_seed();

        if plant_type_seed < 30 {
            let threshold = Self::get_frequency_threshold(0.93, FrequencyTier::Frequent);
            if noise_value > threshold {
                chunk.set_block(x, surface_y, z, BlockType::ShortGrass, Rotation::none());
            }
        } else if plant_type_seed < 50 {
            let threshold = Self::get_frequency_threshold(0.93, FrequencyTier::Moderate);
            if noise_value > threshold {
                chunk.set_block(
                    x,
                    surface_y,
                    z,
                    BlockType::TallGrassBottom,
                    Rotation::none(),
                );
                if surface_y + 1 < CHUNK_HEIGHT {
                    chunk.set_block(
                        x,
                        surface_y + 1,
                        z,
                        BlockType::TallGrassTop,
                        Rotation::none(),
                    );
                }
            }
        } else if plant_type_seed < 70 {
            let threshold = Self::get_frequency_threshold(0.93, FrequencyTier::Moderate);
            if noise_value > threshold {
                chunk.set_block(x, surface_y, z, BlockType::Poppy, Rotation::none());
            }
        } else {
            let threshold = Self::get_frequency_threshold(0.93, FrequencyTier::Moderate);
            if noise_value > threshold {
                chunk.set_block(x, surface_y, z, BlockType::PinkTulip, Rotation::none());
            }
        }
    }

    fn generate_taiga_decoration(
        chunk: &mut Chunk,
        x: i32,
        surface_y: i32,
        z: i32,
        world_x: i32,
        world_z: i32,
    ) {
        let rng = SeededRng::new(world_x, world_z);
        let noise_value = rng.noise_f32();
        let plant_type_seed = rng.plant_type_seed();

        if plant_type_seed < 30 {
            let threshold = Self::get_frequency_threshold(0.94, FrequencyTier::Frequent);
            if noise_value > threshold {
                chunk.set_block(x, surface_y, z, BlockType::ShortGrass, Rotation::none());
            }
        } else if plant_type_seed < 50 {
            let threshold = Self::get_frequency_threshold(0.94, FrequencyTier::Moderate);
            if noise_value > threshold {
                chunk.set_block(x, surface_y, z, BlockType::Fern, Rotation::none());
            }
        } else if plant_type_seed < 70 {
            let threshold = Self::get_frequency_threshold(0.94, FrequencyTier::Moderate);
            if noise_value > threshold {
                chunk.set_block(
                    x,
                    surface_y,
                    z,
                    BlockType::LargeFernBottom,
                    Rotation::none(),
                );
                if surface_y + 1 < CHUNK_HEIGHT {
                    chunk.set_block(
                        x,
                        surface_y + 1,
                        z,
                        BlockType::LargeFernTop,
                        Rotation::none(),
                    );
                }
            }
        } else {
            let threshold = Self::get_frequency_threshold(0.94, FrequencyTier::Rare);
            if noise_value > threshold {
                chunk.set_block(
                    x,
                    surface_y,
                    z,
                    BlockType::SweetBerryBushStage1,
                    Rotation::none(),
                );
            }
        }
    }

    fn generate_podzol_decoration(
        chunk: &mut Chunk,
        x: i32,
        surface_y: i32,
        z: i32,
        world_x: i32,
        world_z: i32,
    ) {
        let rng = SeededRng::new(world_x, world_z);
        let noise_value = rng.noise_f32();

        if noise_value > 0.92 {
            chunk.set_block(x, surface_y, z, BlockType::BrownMushroom, Rotation::none());
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
        let rng = SeededRng::new(world_x, world_z);
        let noise_value = rng.noise_f32();
        let plant_type_seed = rng.plant_type_seed();

        if plant_type_seed < 35 {
            let threshold = Self::get_frequency_threshold(0.99, FrequencyTier::Frequent);
            if noise_value > threshold {
                chunk.set_block(x, surface_y, z, BlockType::ShortDryGrass, Rotation::none());
            }
        } else if plant_type_seed < 65 {
            let threshold = Self::get_frequency_threshold(0.99, FrequencyTier::Moderate);
            if noise_value > threshold {
                chunk.set_block(x, surface_y, z, BlockType::TallDryGrass, Rotation::none());
            }
        } else {
            let threshold = Self::get_frequency_threshold(0.99, FrequencyTier::Rare);
            if noise_value > threshold {
                chunk.set_block(x, surface_y, z, BlockType::DeadBush, Rotation::none());
            }
        }
    }
}
