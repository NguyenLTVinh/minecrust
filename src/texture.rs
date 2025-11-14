use crate::block::{BlockType, FaceDirection};
use gl::types::*;
use image::RgbaImage;

const ATLAS_SIZE: u32 = 48;
const TEXTURE_SIZE: u32 = 16;

#[derive(Clone, Copy)]
pub struct TextureCoords {
    pub u_min: f32,
    pub v_min: f32,
    pub u_max: f32,
    pub v_max: f32,
}

impl TextureCoords {
    fn new(index: usize, atlas_width: u32, atlas_height: u32) -> Self {
        let textures_per_row = (atlas_width / TEXTURE_SIZE) as usize;
        let row = index / textures_per_row;
        let col = index % textures_per_row;

        let u_min = (col * TEXTURE_SIZE as usize) as f32 / atlas_width as f32;
        let v_min = (row * TEXTURE_SIZE as usize) as f32 / atlas_height as f32;
        let u_max = ((col + 1) * TEXTURE_SIZE as usize) as f32 / atlas_width as f32;
        let v_max = ((row + 1) * TEXTURE_SIZE as usize) as f32 / atlas_height as f32;

        TextureCoords {
            u_min,
            v_min,
            u_max,
            v_max,
        }
    }
}

pub struct TextureAtlas {
    pub texture_id: GLuint,
    grass_color: [f32; 3],
    foliage_color: [f32; 3],
    atlas_width: u32,
    atlas_height: u32,
}

impl TextureAtlas {
    pub fn new() -> Result<Self, String> {
        let stone = image::open("textures/block/stone.png")
            .map_err(|e| format!("Failed to load stone.png: {}", e))?;
        let dirt = image::open("textures/block/dirt.png")
            .map_err(|e| format!("Failed to load dirt.png: {}", e))?;
        let grass_side = image::open("textures/block/grass_block_side.png")
            .map_err(|e| format!("Failed to load grass_block_side.png: {}", e))?;
        let grass_top = image::open("textures/block/grass_block_top.png")
            .map_err(|e| format!("Failed to load grass_block_top.png: {}", e))?;
        let oak_log = image::open("textures/block/oak_log.png")
            .map_err(|e| format!("Failed to load oak_log.png: {}", e))?;
        let oak_log_top = image::open("textures/block/oak_log_top.png")
            .map_err(|e| format!("Failed to load oak_log_top.png: {}", e))?;
        let oak_leaves = image::open("textures/block/oak_leaves.png")
            .map_err(|e| format!("Failed to load oak_leaves.png: {}", e))?;
        let water_still = image::open("textures/block/water_still.png")
            .map_err(|e| format!("Failed to load water_still.png: {}", e))?;
        let snow = image::open("textures/block/snow.png")
            .map_err(|e| format!("Failed to load snow.png: {}", e))?;
        let grass_snow = image::open("textures/block/grass_block_snow.png")
            .map_err(|e| format!("Failed to load grass_block_snow.png: {}", e))?;

        let atlas_width = TEXTURE_SIZE * 3;
        let atlas_height = TEXTURE_SIZE * 4;
        let mut atlas = RgbaImage::new(atlas_width, atlas_height);

        let textures = vec![
            stone,
            dirt,
            grass_side,
            grass_top,
            oak_log,
            oak_log_top,
            oak_leaves,
            water_still,
            snow,
            grass_snow,
        ];
        for (i, tex) in textures.iter().enumerate() {
            let tex = tex.to_rgba8();
            let row = i / 3;
            let col = i % 3;
            let x_offset = col as u32 * TEXTURE_SIZE;
            let y_offset = row as u32 * TEXTURE_SIZE;

            for y in 0..TEXTURE_SIZE {
                for x in 0..TEXTURE_SIZE {
                    let pixel = tex.get_pixel(x, y);
                    atlas.put_pixel(x_offset + x, y_offset + y, *pixel);
                }
            }
        }

        let grass_color = Self::load_colormap_sample("textures/colormap/grass.png")?;
        let foliage_color = Self::load_colormap_sample("textures/colormap/foliage.png")?;

        let texture_id = unsafe {
            let mut texture = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                atlas_width as i32,
                atlas_height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                atlas.as_raw().as_ptr() as *const _,
            );

            texture
        };

        Ok(TextureAtlas {
            texture_id,
            grass_color,
            foliage_color,
            atlas_width,
            atlas_height,
        })
    }

    fn load_colormap_sample(path: &str) -> Result<[f32; 3], String> {
        let img = image::open(path).map_err(|e| format!("Failed to load {}: {}", path, e))?;
        let img = img.to_rgba8();
        let (width, height) = img.dimensions();

        let x = width / 2;
        let y = height / 2;
        let pixel = img.get_pixel(x, y);

        Ok([
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        ])
    }

    pub fn get_tex_coords(&self, block: BlockType, face: FaceDirection) -> TextureCoords {
        let index = match block {
            BlockType::Stone => 0,
            BlockType::Dirt => 1,
            BlockType::Grass => match face {
                FaceDirection::Top => 3,
                FaceDirection::Bottom => 1,
                _ => 2,
            },
            BlockType::GrassSnowy => match face {
                FaceDirection::Top => 8,
                FaceDirection::Bottom => 1,
                _ => 9,
            },
            BlockType::Wood => match face {
                FaceDirection::Top | FaceDirection::Bottom => 5,
                _ => 4,
            },
            BlockType::Leaves => 6,
            BlockType::Water => 7,
            BlockType::Snow | BlockType::SnowLayer => 8,
            BlockType::Air => 0,
        };
        TextureCoords::new(index, self.atlas_width, self.atlas_height)
    }

    pub fn get_tint(&self, block: BlockType) -> [f32; 3] {
        match block {
            BlockType::Grass | BlockType::GrassSnowy => self.grass_color,
            BlockType::Leaves => self.foliage_color,
            BlockType::Water => [0.25, 0.5, 0.9],
            _ => [1.0, 1.0, 1.0],
        }
    }
}

impl Drop for TextureAtlas {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}
