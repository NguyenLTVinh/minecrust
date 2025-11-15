pub struct SeededRng {
    seed: u32,
}

impl SeededRng {
    pub fn new(x: i32, z: i32) -> Self {
        let seed =
            ((x as u32).wrapping_mul(374761393)).wrapping_add((z as u32).wrapping_mul(668265263));
        SeededRng { seed }
    }

    pub fn from_seed(seed: u32) -> Self {
        SeededRng { seed }
    }

    pub fn next_f32(&self) -> f32 {
        ((self.seed.wrapping_mul(1664525).wrapping_add(1013904223)) % 1000) as f32 / 1000.0
    }

    pub fn next_u32(&self) -> u32 {
        self.seed.wrapping_mul(1664525).wrapping_add(1013904223)
    }

    pub fn variant(&self, offset: u32) -> SeededRng {
        SeededRng {
            seed: self.seed.wrapping_add(offset),
        }
    }

    pub fn noise_f32(&self) -> f32 {
        (self.seed % 1000) as f32 / 1000.0
    }

    pub fn plant_type_seed(&self) -> u32 {
        let plant_seed = ((self.seed as u32).wrapping_mul(109739919)) % 100;
        plant_seed
    }

    pub fn new_with_y(x: i32, y: i32, z: i32) -> Self {
        let seed = ((x as u32).wrapping_mul(374761393))
            .wrapping_add((y as u32).wrapping_mul(715827883))
            .wrapping_add((z as u32).wrapping_mul(668265263));
        SeededRng { seed }
    }

    pub fn next_mod(&self, modulo: u32) -> u32 {
        self.seed.wrapping_mul(1664525).wrapping_add(1013904223) % modulo
    }

    pub fn next_range(&self, max: i32) -> i32 {
        (self.next_mod(max as u32)) as i32
    }

    pub fn next_range_between(&self, min: i32, max: i32) -> i32 {
        min + self.next_range(max - min)
    }
}
