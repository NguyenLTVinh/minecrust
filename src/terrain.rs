use noise::{Fbm, NoiseFn, Perlin};

pub struct TerrainGenerator {
    terrain_base: Fbm<Perlin>,
    terrain_alt: Fbm<Perlin>,
    height_select: Fbm<Perlin>,
    mountain_height: Fbm<Perlin>,
    mountain_3d: Fbm<Perlin>,
    valley_depth: Fbm<Perlin>,
    valley_profile: Fbm<Perlin>,
    river_noise: Fbm<Perlin>,
    water_level: i32,
    mountain_zero_level: i32,
}

impl TerrainGenerator {
    pub fn new(seed: u32) -> Self {
        let mut terrain_base = Fbm::<Perlin>::new(seed);
        terrain_base.octaves = 4;
        terrain_base.frequency = 1.0 / 800.0;
        terrain_base.lacunarity = 2.0;
        terrain_base.persistence = 0.5;

        let mut terrain_alt = Fbm::<Perlin>::new(seed + 1);
        terrain_alt.octaves = 4;
        terrain_alt.frequency = 1.0 / 800.0;
        terrain_alt.lacunarity = 2.0;
        terrain_alt.persistence = 0.5;

        let mut height_select = Fbm::<Perlin>::new(seed + 2);
        height_select.octaves = 4;
        height_select.frequency = 1.0 / 600.0;
        height_select.lacunarity = 2.0;
        height_select.persistence = 0.6;

        let mut mountain_height = Fbm::<Perlin>::new(seed + 3);
        mountain_height.octaves = 2;
        mountain_height.frequency = 1.0 / 1200.0;
        mountain_height.lacunarity = 2.0;
        mountain_height.persistence = 0.5;

        let mut mountain_3d = Fbm::<Perlin>::new(seed + 4);
        mountain_3d.octaves = 4;
        mountain_3d.frequency = 1.0 / 350.0;
        mountain_3d.lacunarity = 2.0;
        mountain_3d.persistence = 0.55;

        let mut valley_depth = Fbm::<Perlin>::new(seed + 5);
        valley_depth.octaves = 1;
        valley_depth.frequency = 1.0 / 700.0;
        valley_depth.lacunarity = 2.0;
        valley_depth.persistence = 1.0;

        let mut valley_profile = Fbm::<Perlin>::new(seed + 6);
        valley_profile.octaves = 1;
        valley_profile.frequency = 1.0 / 700.0;
        valley_profile.lacunarity = 2.0;
        valley_profile.persistence = 1.0;

        let mut river_noise = Fbm::<Perlin>::new(seed + 7);
        river_noise.octaves = 3;
        river_noise.frequency = 1.0 / 400.0;
        river_noise.lacunarity = 2.0;
        river_noise.persistence = 0.5;

        TerrainGenerator {
            terrain_base,
            terrain_alt,
            height_select,
            mountain_height,
            mountain_3d,
            valley_depth,
            valley_profile,
            river_noise,
            water_level: 32,
            mountain_zero_level: 45,
        }
    }

    fn get_base_terrain_height(&self, x: f64, z: f64) -> f64 {
        let h_select = self.height_select.get([x, z]);
        let h_select = (h_select * 0.5 + 0.5).clamp(0.0, 1.0);

        let height_base = self.terrain_base.get([x, z]) * 35.0 + 8.0;
        let height_alt = self.terrain_alt.get([x, z]) * 15.0 + 8.0;

        let height = if height_alt > height_base {
            height_alt
        } else {
            height_base * h_select + height_alt * (1.0 - h_select)
        };

        height
    }

    fn get_valley_terrain(&self, x: f64, z: f64) -> (f64, f64, f64) {
        let n_valley = self.valley_depth.get([x, z]) * 2.0 + 3.0;
        let n_valley_profile = self.valley_profile.get([x, z]) * 0.3 + 0.5;
        let n_rivers = self.river_noise.get([x, z]);

        let valley_d = n_valley * n_valley * 0.3;
        let river = n_rivers.abs() - 0.08;

        // Exponential curve models valley shape
        let tv = (river / n_valley_profile).max(0.0);
        let valley_h = valley_d * (1.0 - (-tv * tv).exp());

        (valley_d, valley_h, n_rivers)
    }

    fn is_mountain_terrain(&self, x: f64, y: f64, z: f64) -> bool {
        let mnt_h = (self.mountain_height.get([x, z]) * 40.0 + 80.0).max(1.0);

        // Density gradient causes mountains to thin out with altitude
        let density_gradient = -((y - self.mountain_zero_level as f64) / mnt_h);
        let mnt_n = self.mountain_3d.get([x, y, z]) * 0.7;

        mnt_n + density_gradient >= 0.0
    }

    fn is_river_channel(&self, _x: f64, _z: f64, y: f64, river_noise: f64) -> bool {
        let river_width = 0.08;
        let abs_river = river_noise.abs();

        if abs_river > river_width {
            return false;
        }

        let altitude = y - self.water_level as f64;
        let height_mod = (altitude + 10.0) / 4.0;
        let width_mod = river_width - abs_river;

        width_mod * height_mod >= 0.02
    }

    pub fn get_terrain_height(&self, world_x: i32, world_z: i32) -> i32 {
        let x = world_x as f64;
        let z = world_z as f64;

        let base_height = self.get_base_terrain_height(x, z);
        let (valley_d, valley_h, n_rivers) = self.get_valley_terrain(x, z);

        let base = base_height + valley_d;
        let surface_y = base + valley_h;

        let river_depth = 2.5;
        let is_river = n_rivers.abs() < 0.08;

        let final_height = if is_river {
            let river_y = base - river_depth;
            river_y.max(self.water_level as f64 - 2.0).min(surface_y)
        } else {
            surface_y
        };

        final_height as i32
    }

    pub fn is_solid_at(&self, world_x: i32, world_y: i32, world_z: i32) -> bool {
        let x = world_x as f64;
        let y = world_y as f64;
        let z = world_z as f64;

        let base_height = self.get_base_terrain_height(x, z);
        let (valley_d, valley_h, n_rivers) = self.get_valley_terrain(x, z);

        let base = base_height + valley_d;
        let surface_y = base + valley_h;

        let in_mountain = self.is_mountain_terrain(x, y, z);
        if in_mountain && y >= surface_y {
            return true;
        }

        let is_river = if !in_mountain {
            self.is_river_channel(x, z, y, n_rivers)
        } else {
            false
        };

        if y <= surface_y && !is_river {
            return true;
        }

        false
    }

    pub fn is_water_at(&self, world_x: i32, world_y: i32, world_z: i32) -> bool {
        !self.is_solid_at(world_x, world_y, world_z) && world_y <= self.water_level
    }

    pub fn get_water_level(&self) -> i32 {
        self.water_level
    }
}
