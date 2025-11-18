use crate::block::{BlockType, FaceDirection, Rotation};
use gl::types::*;
use image::{DynamicImage, RgbaImage};
use std::path::PathBuf;

const TEXTURE_SIZE: u32 = 16;

fn get_texture_path(relative_path: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .join("textures")
        .join(relative_path)
}

fn create_fallback_texture() -> DynamicImage {
    let mut img = RgbaImage::new(TEXTURE_SIZE, TEXTURE_SIZE);
    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            img.put_pixel(x, y, image::Rgba([255, 0, 255, 255]));
        }
    }
    DynamicImage::ImageRgba8(img)
}

fn load_texture(relative_path: &str) -> DynamicImage {
    let full_path = get_texture_path(relative_path);
    match image::open(&full_path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!(
                "Warning: Failed to load texture {}: {}. Using purple fallback.",
                full_path.display(),
                e
            );
            create_fallback_texture()
        }
    }
}

#[derive(Clone, Copy)]
pub struct TextureCoords {
    pub u_min: f32,
    pub v_min: f32,
    pub u_max: f32,
    pub v_max: f32,
}

impl TextureCoords {
    pub fn new(index: usize, atlas_width: u32, atlas_height: u32) -> Self {
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

    pub fn get_uvs_for_face(&self, face_dir: FaceDirection, rotation: Rotation) -> [[f32; 2]; 6] {
        let mut uvs = match face_dir {
            FaceDirection::Top => [
                [self.u_min, self.v_min],
                [self.u_min, self.v_max],
                [self.u_max, self.v_max],
                [self.u_min, self.v_min],
                [self.u_max, self.v_max],
                [self.u_max, self.v_min],
            ],
            FaceDirection::Bottom => [
                [self.u_min, self.v_min],
                [self.u_min, self.v_max],
                [self.u_max, self.v_max],
                [self.u_min, self.v_min],
                [self.u_max, self.v_max],
                [self.u_max, self.v_min],
            ],
            FaceDirection::Front => [
                [self.u_min, self.v_max],
                [self.u_min, self.v_min],
                [self.u_max, self.v_min],
                [self.u_min, self.v_max],
                [self.u_max, self.v_min],
                [self.u_max, self.v_max],
            ],
            FaceDirection::Back => [
                [self.u_max, self.v_max],
                [self.u_min, self.v_max],
                [self.u_min, self.v_min],
                [self.u_max, self.v_max],
                [self.u_min, self.v_min],
                [self.u_max, self.v_min],
            ],
            FaceDirection::Right => [
                [self.u_min, self.v_max],
                [self.u_min, self.v_min],
                [self.u_max, self.v_min],
                [self.u_min, self.v_max],
                [self.u_max, self.v_min],
                [self.u_max, self.v_max],
            ],
            FaceDirection::Left => [
                [self.u_max, self.v_max],
                [self.u_min, self.v_max],
                [self.u_min, self.v_min],
                [self.u_max, self.v_max],
                [self.u_min, self.v_min],
                [self.u_max, self.v_min],
            ],
        };
        uvs
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
        let texture_paths = vec![
            "block/stone.png",
            "block/dirt.png",
            "block/grass_block_side.png",
            "block/grass_block_top.png",
            "block/oak_log.png",
            "block/oak_log_top.png",
            "block/oak_leaves.png",
            "block/water_still.png",
            "block/snow.png",
            "block/grass_block_snow.png",
            "block/sand.png",
            "block/spruce_log.png",
            "block/spruce_log_top.png",
            "block/spruce_leaves.png",
            "block/birch_log.png",
            "block/birch_log_top.png",
            "block/birch_leaves.png",
            "block/brown_mushroom.png",
            "block/poppy.png",
            "block/short_grass.png",
            "block/tall_grass_top.png",
            "block/dead_bush.png",
            "block/red_mushroom.png",
            "block/tall_dry_grass.png",
            "block/torchflower.png",
            "block/pink_tulip.png",
            "block/short_dry_grass.png",
            "block/tall_grass_bottom.png",
            "block/podzol_top.png",
            "block/podzol_side.png",
            "block/fern.png",
            "block/large_fern_top.png",
            "block/large_fern_bottom.png",
            "block/sweet_berry_bush_stage2.png",
            "block/sweet_berry_bush_stage3.png",
            "block/cactus_side.png",
            "block/cactus_top.png",
            "block/cactus_bottom.png",
            "block/cactus_flower.png",
            "block/jungle_log.png",
            "block/jungle_log_top.png",
            "block/jungle_leaves.png",
            "block/oak_planks.png",
            "block/spruce_planks.png",
            "block/birch_planks.png",
            "block/jungle_planks.png",
            "block/acacia_planks.png",
            "block/dark_oak_planks.png",
            "block/mangrove_planks.png",
            "block/cherry_planks.png",
            "block/bamboo_planks.png",
            "block/crimson_planks.png",
            "block/warped_planks.png",
            "block/pale_oak_planks.png",
            "block/cobblestone.png",
            "block/mossy_cobblestone.png",
            "block/stone_bricks.png",
            "block/chiseled_stone_bricks.png",
            "block/mossy_stone_bricks.png",
            "block/smooth_stone.png",
            "block/sandstone.png",
            "block/sandstone_top.png",
            "block/sandstone_bottom.png",
            "block/chiseled_sandstone.png",
            "block/terracotta.png",
            "block/black_terracotta.png",
            "block/blue_terracotta.png",
            "block/brown_terracotta.png",
            "block/cyan_terracotta.png",
            "block/gray_terracotta.png",
            "block/green_terracotta.png",
            "block/light_blue_terracotta.png",
            "block/light_gray_terracotta.png",
            "block/lime_terracotta.png",
            "block/magenta_terracotta.png",
            "block/orange_terracotta.png",
            "block/pink_terracotta.png",
            "block/purple_terracotta.png",
            "block/red_terracotta.png",
            "block/white_terracotta.png",
            "block/yellow_terracotta.png",
            "block/black_glazed_terracotta.png",
            "block/blue_glazed_terracotta.png",
            "block/brown_glazed_terracotta.png",
            "block/cyan_glazed_terracotta.png",
            "block/gray_glazed_terracotta.png",
            "block/green_glazed_terracotta.png",
            "block/light_blue_glazed_terracotta.png",
            "block/light_gray_glazed_terracotta.png",
            "block/lime_glazed_terracotta.png",
            "block/magenta_glazed_terracotta.png",
            "block/orange_glazed_terracotta.png",
            "block/pink_glazed_terracotta.png",
            "block/purple_glazed_terracotta.png",
            "block/red_glazed_terracotta.png",
            "block/white_glazed_terracotta.png",
            "block/yellow_glazed_terracotta.png",
            "block/copper_block.png",
            "block/diamond_block.png",
            "block/emerald_block.png",
            "block/gold_block.png",
            "block/iron_block.png",
            "block/lapis_block.png",
        ];

        let atlas_width = TEXTURE_SIZE * 16;
        let atlas_height = TEXTURE_SIZE * 8;

        let mut atlas = RgbaImage::new(atlas_width, atlas_height);

        for (i, path) in texture_paths.iter().enumerate() {
            let tex = load_texture(path).to_rgba8();
            let row = i / 16;
            let col = i % 16;

            let x_offset = col as u32 * TEXTURE_SIZE;
            let y_offset = row as u32 * TEXTURE_SIZE;

            for y in 0..TEXTURE_SIZE {
                for x in 0..TEXTURE_SIZE {
                    let pixel = tex.get_pixel(x, y);
                    atlas.put_pixel(x_offset + x, y_offset + y, *pixel);
                }
            }
        }

        let grass_color = Self::load_colormap_sample("colormap/grass.png")?;
        let foliage_color = Self::load_colormap_sample("colormap/foliage.png")?;

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
        let full_path = get_texture_path(path);
        let img = image::open(&full_path)
            .map_err(|e| format!("Failed to load {}: {}", full_path.display(), e))?;
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

    pub fn get_tex_coords(&self, texture_index: usize) -> TextureCoords {
        TextureCoords::new(texture_index, self.atlas_width, self.atlas_height)
    }

    pub fn get_tint(&self, block: BlockType) -> [f32; 3] {
        match block {
            BlockType::GrassBlock => self.grass_color,
            BlockType::ShortGrass | BlockType::TallGrassTop | BlockType::TallGrassBottom => {
                self.grass_color
            }
            BlockType::OakLeaves
            | BlockType::SpruceLeaves
            | BlockType::BirchLeaves
            | BlockType::JungleLeaves
            | BlockType::Fern
            | BlockType::LargeFernBottom
            | BlockType::LargeFernTop => self.foliage_color,
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
