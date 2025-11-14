use crate::block::BlockType;

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
        }
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
            60,
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
            30,
            8,
        ));

        self.register(Biome::new(
            "taiga",
            BlockType::GrassSnowy,
            BlockType::Dirt,
            BlockType::Stone,
            BlockType::Water,
            BlockType::Water,
            1,
            3,
            20.0,
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
            255,
            16,
        ));

        self.register(Biome::new(
            "forest",
            BlockType::Grass,
            BlockType::Dirt,
            BlockType::Stone,
            BlockType::Water,
            BlockType::Water,
            1,
            3,
            40.0,
            80.0,
            0,
            255,
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
        let rng_val = ((seed.wrapping_mul(1664525).wrapping_add(1013904223)) >> 1) as i32;

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
