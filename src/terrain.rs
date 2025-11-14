use crate::biome::{Biome, BiomeManager};
use noise::{Fbm, NoiseFn, Perlin};

const CHUNK_HEIGHT: i32 = 160;
const MAX_MOUNTAIN_HEIGHT: i32 = CHUNK_HEIGHT - 10;

pub struct MapGenerator {
    pub seed: u32,
    pub water_level: i32,
    pub mount_zero_level: i32,

    terrain_base: Fbm<Perlin>,
    terrain_alt: Fbm<Perlin>,
    terrain_persist: Fbm<Perlin>,
    height_select: Fbm<Perlin>,
    filler_depth: Fbm<Perlin>,
    mount_height: Fbm<Perlin>,
    mountain: Fbm<Perlin>,
    ridge: Fbm<Perlin>,
    ridge_uwater: Fbm<Perlin>,

    heat: Fbm<Perlin>,
    heat_blend: Fbm<Perlin>,
    humidity: Fbm<Perlin>,
    humidity_blend: Fbm<Perlin>,

    biome_manager: BiomeManager,
}

impl MapGenerator {
    pub fn new(seed: u32) -> Self {
        let mut terrain_base = Fbm::new(seed);
        terrain_base.frequency = 1.0 / 600.0;
        terrain_base.lacunarity = 2.0;
        terrain_base.persistence = 0.6;
        terrain_base.octaves = 5;

        let mut terrain_alt = Fbm::new(seed.wrapping_add(1));
        terrain_alt.frequency = 1.0 / 600.0;
        terrain_alt.lacunarity = 2.0;
        terrain_alt.persistence = 0.6;
        terrain_alt.octaves = 5;

        let mut terrain_persist = Fbm::new(seed.wrapping_add(2));
        terrain_persist.frequency = 1.0 / 2000.0;
        terrain_persist.lacunarity = 2.0;
        terrain_persist.persistence = 0.6;
        terrain_persist.octaves = 3;

        let mut height_select = Fbm::new(seed.wrapping_add(3));
        height_select.frequency = 1.0 / 500.0;
        height_select.lacunarity = 2.0;
        height_select.persistence = 0.7;
        height_select.octaves = 6;

        let mut filler_depth = Fbm::new(seed.wrapping_add(4));
        filler_depth.frequency = 1.0 / 150.0;
        filler_depth.lacunarity = 2.0;
        filler_depth.persistence = 0.7;
        filler_depth.octaves = 3;

        let mut mount_height = Fbm::new(seed.wrapping_add(5));
        mount_height.frequency = 1.0 / 1000.0;
        mount_height.lacunarity = 2.0;
        mount_height.persistence = 0.3;
        mount_height.octaves = 3;

        let mut mountain = Fbm::new(seed.wrapping_add(6));
        mountain.frequency = 1.0 / 250.0;
        mountain.lacunarity = 2.0;
        mountain.persistence = 0.3;
        mountain.octaves = 3;

        let mut ridge = Fbm::new(seed.wrapping_add(7));
        ridge.frequency = 1.0 / 100.0;
        ridge.lacunarity = 2.0;
        ridge.persistence = 0.75;
        ridge.octaves = 4;

        let mut ridge_uwater = Fbm::new(seed.wrapping_add(8));
        ridge_uwater.frequency = 1.0 / 1000.0;
        ridge_uwater.lacunarity = 2.0;
        ridge_uwater.persistence = 0.6;
        ridge_uwater.octaves = 5;

        let mut heat = Fbm::new(seed.wrapping_add(9));
        heat.frequency = 1.0 / 1000.0;
        heat.lacunarity = 2.0;
        heat.persistence = 0.5;
        heat.octaves = 3;

        let mut heat_blend = Fbm::new(seed.wrapping_add(10));
        heat_blend.frequency = 1.0 / 8.0;
        heat_blend.lacunarity = 2.0;
        heat_blend.persistence = 1.0;
        heat_blend.octaves = 2;

        let mut humidity = Fbm::new(seed.wrapping_add(11));
        humidity.frequency = 1.0 / 1000.0;
        humidity.lacunarity = 2.0;
        humidity.persistence = 0.5;
        humidity.octaves = 3;

        let mut humidity_blend = Fbm::new(seed.wrapping_add(12));
        humidity_blend.frequency = 1.0 / 8.0;
        humidity_blend.lacunarity = 2.0;
        humidity_blend.persistence = 1.0;
        humidity_blend.octaves = 2;

        MapGenerator {
            seed,
            water_level: 1,
            mount_zero_level: 35,

            terrain_base,
            terrain_alt,
            terrain_persist,
            height_select,
            filler_depth,
            mount_height,
            mountain,
            ridge,
            ridge_uwater,

            heat,
            heat_blend,
            humidity,
            humidity_blend,

            biome_manager: BiomeManager::new(),
        }
    }

    pub fn get_water_level(&self) -> i32 {
        self.water_level
    }

    fn base_terrain_level(&self, x: f64, z: f64) -> f32 {
        let hselect = self.height_select.get([x, z]);
        let hselect = (hselect * 0.5 + 0.5).max(0.0).min(1.0) as f32;

        let persist = self.terrain_persist.get([x, z]) as f32;

        let height_base = (self.terrain_base.get([x, z]) as f32) * 70.0 * persist.max(0.1);
        let height_alt = (self.terrain_alt.get([x, z]) as f32) * 25.0 * persist.max(0.1);

        if height_alt > height_base {
            height_alt + 4.0
        } else {
            (height_base * hselect) + (height_alt * (1.0 - hselect)) + 4.0
        }
    }

    fn get_mountain_terrain(&self, x: f64, y: f64, z: f64) -> bool {
        if y as i32 >= MAX_MOUNTAIN_HEIGHT {
            return false;
        }

        let mnt_h = ((self.mount_height.get([x, z]) as f32) * 60.0 + 60.0).max(1.0);

        let max_height = (MAX_MOUNTAIN_HEIGHT - self.mount_zero_level) as f32;
        let clamped_mnt_h = mnt_h.min(max_height);

        let density_gradient = -((y as f32 - self.mount_zero_level as f32) / clamped_mnt_h);
        let mnt_n = (self.mountain.get([x, y, z]) as f32) * 1.0;

        mnt_n + density_gradient >= 0.0
    }

    fn get_river_channel(&self, x: f64, z: f64, y: f64) -> bool {
        let width = 0.2;
        let uwater = self.ridge_uwater.get([x, z]);
        let abs_uwater = uwater.abs() * 2.0;

        if abs_uwater > width {
            return false;
        }

        let altitude = y as f32 - self.water_level as f32;
        let height_mod = (altitude + 17.0) / 2.5;
        let width_mod = (width - abs_uwater) as f32;
        let nridge = (self.ridge.get([x, y, z]) as f32) * 1.0;

        let actual_ridge = nridge * altitude.max(0.0) / 7.0;
        actual_ridge + width_mod * height_mod >= 0.6
    }

    pub fn get_terrain_height(&self, world_x: i32, world_z: i32) -> i32 {
        let x = world_x as f64;
        let z = world_z as f64;

        let base_height = self.base_terrain_level(x, z);
        let base = base_height as i32;

        if self.get_mountain_terrain(x, base as f64, z) {
            base
        } else {
            base.max(self.water_level)
        }
    }

    pub fn is_solid_at(&self, world_x: i32, world_y: i32, world_z: i32) -> bool {
        if world_y >= CHUNK_HEIGHT {
            return false;
        }

        let x = world_x as f64;
        let y = world_y as f64;
        let z = world_z as f64;

        let base_height = self.base_terrain_level(x, z);

        if y <= base_height as f64 {
            let is_river = self.get_river_channel(x, z, y);
            return !is_river;
        }

        if self.get_mountain_terrain(x, y, z) {
            return true;
        }

        false
    }

    pub fn is_water_at(&self, world_x: i32, world_y: i32, world_z: i32) -> bool {
        if world_y > self.water_level || world_y >= CHUNK_HEIGHT {
            return false;
        }

        let x = world_x as f64;
        let z = world_z as f64;
        let y = world_y as f64;

        if self.get_river_channel(x, z, y) {
            return true;
        }

        if self.is_solid_at(world_x, world_y, world_z) {
            return false;
        }

        world_y <= self.water_level
    }

    pub fn get_heat(&self, x: f64, z: f64) -> f32 {
        let h = (self.heat.get([x, z]) as f32) * 50.0;
        let hb = (self.heat_blend.get([x, z]) as f32) * 1.5;
        h + hb + 50.0
    }

    pub fn get_humidity(&self, x: f64, z: f64) -> f32 {
        let h = (self.humidity.get([x, z]) as f32) * 50.0;
        let hb = (self.humidity_blend.get([x, z]) as f32) * 1.5;
        h + hb + 50.0
    }

    pub fn get_biome_at(&self, x: f64, y: i32, z: f64) -> &Biome {
        let heat = self.get_heat(x, z);
        let humidity = self.get_humidity(x, z);
        self.biome_manager.get_biome_at_pos(heat, humidity, y)
    }

    pub fn get_filler_depth(&self, x: f64, z: f64) -> i32 {
        let depth = (self.filler_depth.get([x, z]) as f32) * 1.2;
        (depth * 5.0).max(0.0) as i32
    }
}

pub struct TerrainGenerator {
    mapgen: MapGenerator,
}

impl TerrainGenerator {
    pub fn new(seed: u32) -> Self {
        TerrainGenerator {
            mapgen: MapGenerator::new(seed),
        }
    }

    pub fn get_terrain_height(&self, world_x: i32, world_z: i32) -> i32 {
        self.mapgen.get_terrain_height(world_x, world_z)
    }

    pub fn is_solid_at(&self, world_x: i32, world_y: i32, world_z: i32) -> bool {
        self.mapgen.is_solid_at(world_x, world_y, world_z)
    }

    pub fn is_water_at(&self, world_x: i32, world_y: i32, world_z: i32) -> bool {
        self.mapgen.is_water_at(world_x, world_y, world_z)
    }

    pub fn get_water_level(&self) -> i32 {
        self.mapgen.get_water_level()
    }

    pub fn get_biome_at(&self, x: i32, y: i32, z: i32) -> &Biome {
        self.mapgen.get_biome_at(x as f64, y as i32, z as f64)
    }

    pub fn get_filler_depth(&self, x: i32, z: i32) -> i32 {
        self.mapgen.get_filler_depth(x as f64, z as f64)
    }
}
