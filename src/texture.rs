use crate::block::{BlockType, FaceDirection};
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

    pub fn get_uvs_for_face(&self, dx: i32, dy: i32, dz: i32) -> [[f32; 2]; 6] {
        match (dx, dy, dz) {
            (-1, 0, 0) => [
                [self.u_max, self.v_max],
                [self.u_min, self.v_max],
                [self.u_min, self.v_min],
                [self.u_max, self.v_max],
                [self.u_min, self.v_min],
                [self.u_max, self.v_min],
            ],
            (1, 0, 0) => [
                [self.u_min, self.v_max],
                [self.u_min, self.v_min],
                [self.u_max, self.v_min],
                [self.u_min, self.v_max],
                [self.u_max, self.v_min],
                [self.u_max, self.v_max],
            ],
            (0, 0, -1) => [
                [self.u_min, self.v_max],
                [self.u_max, self.v_max],
                [self.u_max, self.v_min],
                [self.u_min, self.v_max],
                [self.u_max, self.v_min],
                [self.u_min, self.v_min],
            ],
            (0, 0, 1) => [
                [self.u_min, self.v_max],
                [self.u_min, self.v_min],
                [self.u_max, self.v_min],
                [self.u_min, self.v_max],
                [self.u_max, self.v_min],
                [self.u_max, self.v_max],
            ],
            _ => [
                [self.u_min, self.v_min],
                [self.u_min, self.v_max],
                [self.u_max, self.v_max],
                [self.u_min, self.v_min],
                [self.u_max, self.v_max],
                [self.u_max, self.v_min],
            ],
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
        let stone = load_texture("block/stone.png");
        let dirt = load_texture("block/dirt.png");
        let grass_side = load_texture("block/grass_block_side.png");
        let grass_top = load_texture("block/grass_block_top.png");
        let oak_log = load_texture("block/oak_log.png");
        let oak_log_top = load_texture("block/oak_log_top.png");
        let oak_leaves = load_texture("block/oak_leaves.png");
        let water_still = load_texture("block/water_still.png");
        let snow = load_texture("block/snow.png");
        let grass_snow = load_texture("block/grass_block_snow.png");
        let sand = load_texture("block/sand.png");
        let spruce_log = load_texture("block/spruce_log.png");
        let spruce_log_top = load_texture("block/spruce_log_top.png");
        let spruce_leaves = load_texture("block/spruce_leaves.png");
        let birch_log = load_texture("block/birch_log.png");
        let birch_log_top = load_texture("block/birch_log_top.png");
        let birch_leaves = load_texture("block/birch_leaves.png");
        let brown_mushroom = load_texture("block/brown_mushroom.png");
        let poppy = load_texture("block/poppy.png");
        let short_grass = load_texture("block/short_grass.png");
        let tall_grass_top = load_texture("block/tall_grass_top.png");
        let dead_bush = load_texture("block/dead_bush.png");
        let red_mushroom = load_texture("block/red_mushroom.png");
        let tall_dry_grass = load_texture("block/tall_dry_grass.png");
        let torchflower = load_texture("block/torchflower.png");
        let pink_tulip = load_texture("block/pink_tulip.png");
        let short_dry_grass = load_texture("block/short_dry_grass.png");
        let tall_grass_bottom = load_texture("block/tall_grass_bottom.png");
        let podzol_top = load_texture("block/podzol_top.png");
        let podzol_side = load_texture("block/podzol_side.png");
        let fern = load_texture("block/fern.png");
        let large_fern_top = load_texture("block/large_fern_top.png");
        let large_fern_bottom = load_texture("block/large_fern_bottom.png");
        let sweet_berry_stage1 = load_texture("block/sweet_berry_bush_stage2.png");
        let sweet_berry_stage2 = load_texture("block/sweet_berry_bush_stage3.png");
        let cactus_side = load_texture("block/cactus_side.png");
        let cactus_top = load_texture("block/cactus_top.png");
        let cactus_bottom = load_texture("block/cactus_bottom.png");
        let cactus_flower = load_texture("block/cactus_flower.png");
        let jungle_log = load_texture("block/jungle_log.png");
        let jungle_log_top = load_texture("block/jungle_log_top.png");
        let jungle_leaves = load_texture("block/jungle_leaves.png");
        let oak_planks = load_texture("block/oak_planks.png");
        let spruce_planks = load_texture("block/spruce_planks.png");
        let birch_planks = load_texture("block/birch_planks.png");
        let jungle_planks = load_texture("block/jungle_planks.png");
        let acacia_planks = load_texture("block/acacia_planks.png");
        let dark_oak_planks = load_texture("block/dark_oak_planks.png");
        let mangrove_planks = load_texture("block/mangrove_planks.png");
        let cherry_planks = load_texture("block/cherry_planks.png");
        let bamboo_planks = load_texture("block/bamboo_planks.png");
        let crimson_planks = load_texture("block/crimson_planks.png");
        let warped_planks = load_texture("block/warped_planks.png");
        let pale_oak_planks = load_texture("block/pale_oak_planks.png");
        let cobblestone = load_texture("block/cobblestone.png");
        let mossy_cobblestone = load_texture("block/mossy_cobblestone.png");
        let stone_bricks = load_texture("block/stone_bricks.png");
        let chiseled_stone_bricks = load_texture("block/chiseled_stone_bricks.png");
        let mossy_stone_bricks = load_texture("block/mossy_stone_bricks.png");
        let smooth_stone = load_texture("block/smooth_stone.png");
        let sandstone = load_texture("block/sandstone.png");
        let sandstone_top = load_texture("block/sandstone_top.png");
        let sandstone_bottom = load_texture("block/sandstone_bottom.png");
        let chiseled_sandstone = load_texture("block/chiseled_sandstone.png");
        let terracotta = load_texture("block/terracotta.png");
        let black_terracotta = load_texture("block/black_terracotta.png");
        let blue_terracotta = load_texture("block/blue_terracotta.png");
        let brown_terracotta = load_texture("block/brown_terracotta.png");
        let cyan_terracotta = load_texture("block/cyan_terracotta.png");
        let gray_terracotta = load_texture("block/gray_terracotta.png");
        let green_terracotta = load_texture("block/green_terracotta.png");
        let light_blue_terracotta = load_texture("block/light_blue_terracotta.png");
        let light_gray_terracotta = load_texture("block/light_gray_terracotta.png");
        let lime_terracotta = load_texture("block/lime_terracotta.png");
        let magenta_terracotta = load_texture("block/magenta_terracotta.png");
        let orange_terracotta = load_texture("block/orange_terracotta.png");
        let pink_terracotta = load_texture("block/pink_terracotta.png");
        let purple_terracotta = load_texture("block/purple_terracotta.png");
        let red_terracotta = load_texture("block/red_terracotta.png");
        let white_terracotta = load_texture("block/white_terracotta.png");
        let yellow_terracotta = load_texture("block/yellow_terracotta.png");
        let black_glazed_terracotta = load_texture("block/black_glazed_terracotta.png");
        let blue_glazed_terracotta = load_texture("block/blue_glazed_terracotta.png");
        let brown_glazed_terracotta = load_texture("block/brown_glazed_terracotta.png");
        let cyan_glazed_terracotta = load_texture("block/cyan_glazed_terracotta.png");
        let gray_glazed_terracotta = load_texture("block/gray_glazed_terracotta.png");
        let green_glazed_terracotta = load_texture("block/green_glazed_terracotta.png");
        let light_blue_glazed_terracotta = load_texture("block/light_blue_glazed_terracotta.png");
        let light_gray_glazed_terracotta = load_texture("block/light_gray_glazed_terracotta.png");
        let lime_glazed_terracotta = load_texture("block/lime_glazed_terracotta.png");
        let magenta_glazed_terracotta = load_texture("block/magenta_glazed_terracotta.png");
        let orange_glazed_terracotta = load_texture("block/orange_glazed_terracotta.png");
        let pink_glazed_terracotta = load_texture("block/pink_glazed_terracotta.png");
        let purple_glazed_terracotta = load_texture("block/purple_glazed_terracotta.png");
        let red_glazed_terracotta = load_texture("block/red_glazed_terracotta.png");
        let white_glazed_terracotta = load_texture("block/white_glazed_terracotta.png");
        let yellow_glazed_terracotta = load_texture("block/yellow_glazed_terracotta.png");
        let copper_block = load_texture("block/copper_block.png");
        let diamond_block = load_texture("block/diamond_block.png");
        let emerald_block = load_texture("block/emerald_block.png");
        let gold_block = load_texture("block/gold_block.png");
        let iron_block = load_texture("block/iron_block.png");
        let lapis_block = load_texture("block/lapis_block.png");

        let atlas_width = TEXTURE_SIZE * 16;
        let atlas_height = TEXTURE_SIZE * 8;

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
            sand,
            spruce_log,
            spruce_log_top,
            spruce_leaves,
            birch_log,
            birch_log_top,
            birch_leaves,
            brown_mushroom,
            poppy,
            short_grass,
            tall_grass_top,
            dead_bush,
            red_mushroom,
            tall_dry_grass,
            torchflower,
            pink_tulip,
            short_dry_grass,
            tall_grass_bottom,
            podzol_top,
            podzol_side,
            fern,
            large_fern_top,
            large_fern_bottom,
            sweet_berry_stage1,
            sweet_berry_stage2,
            cactus_side,
            cactus_top,
            cactus_bottom,
            cactus_flower,
            jungle_log,
            jungle_log_top,
            jungle_leaves,
            oak_planks,
            spruce_planks,
            birch_planks,
            jungle_planks,
            acacia_planks,
            dark_oak_planks,
            mangrove_planks,
            cherry_planks,
            bamboo_planks,
            crimson_planks,
            warped_planks,
            pale_oak_planks,
            cobblestone,
            mossy_cobblestone,
            stone_bricks,
            chiseled_stone_bricks,
            mossy_stone_bricks,
            smooth_stone,
            sandstone,
            sandstone_top,
            sandstone_bottom,
            chiseled_sandstone,
            terracotta,
            black_terracotta,
            blue_terracotta,
            brown_terracotta,
            cyan_terracotta,
            gray_terracotta,
            green_terracotta,
            light_blue_terracotta,
            light_gray_terracotta,
            lime_terracotta,
            magenta_terracotta,
            orange_terracotta,
            pink_terracotta,
            purple_terracotta,
            red_terracotta,
            white_terracotta,
            yellow_terracotta,
            black_glazed_terracotta,
            blue_glazed_terracotta,
            brown_glazed_terracotta,
            cyan_glazed_terracotta,
            gray_glazed_terracotta,
            green_glazed_terracotta,
            light_blue_glazed_terracotta,
            light_gray_glazed_terracotta,
            lime_glazed_terracotta,
            magenta_glazed_terracotta,
            orange_glazed_terracotta,
            pink_glazed_terracotta,
            purple_glazed_terracotta,
            red_glazed_terracotta,
            white_glazed_terracotta,
            yellow_glazed_terracotta,
            copper_block,
            diamond_block,
            emerald_block,
            gold_block,
            iron_block,
            lapis_block,
        ];
        for (i, tex) in textures.iter().enumerate() {
            let tex = tex.to_rgba8();
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
            BlockType::OakLog => match face {
                FaceDirection::Top | FaceDirection::Bottom => 5,
                _ => 4,
            },
            BlockType::OakLeaves => 6,
            BlockType::Water => 7,
            BlockType::Snow | BlockType::SnowLayer => 8,
            BlockType::Sand => 10,
            BlockType::SpruceLog => match face {
                FaceDirection::Top | FaceDirection::Bottom => 12,
                _ => 11,
            },
            BlockType::SpruceLeaves => 13,
            BlockType::BirchLog => match face {
                FaceDirection::Top | FaceDirection::Bottom => 15,
                _ => 14,
            },
            BlockType::BirchLeaves => 16,
            BlockType::BrownMushroom => 17,
            BlockType::Poppy => 18,
            BlockType::ShortGrass => 19,
            BlockType::TallGrassTop => 20,
            BlockType::DeadBush => 21,
            BlockType::RedMushroom => 22,
            BlockType::TallDryGrass => 23,
            BlockType::TorchFlower => 24,
            BlockType::PinkTulip => 25,
            BlockType::ShortDryGrass => 26,
            BlockType::TallGrassBottom => 27,
            BlockType::Podzol => match face {
                FaceDirection::Top => 28,
                FaceDirection::Bottom => 1,
                _ => 29,
            },
            BlockType::Fern => 30,
            BlockType::LargeFernTop => 31,
            BlockType::LargeFernBottom => 32,
            BlockType::SweetBerryBushStage1 => 33,
            BlockType::SweetBerryBushStage2 => 34,
            BlockType::Cactus => match face {
                FaceDirection::Top | FaceDirection::Bottom => 36,
                _ => 35,
            },
            BlockType::CactusFlower => 38,
            BlockType::JungleLog => match face {
                FaceDirection::Top | FaceDirection::Bottom => 40,
                _ => 39,
            },
            BlockType::JungleLeaves => 41,
            BlockType::OakPlanks => 42,
            BlockType::Spruceplanks => 43,
            BlockType::BirchPlanks => 44,
            BlockType::JunglePlanks => 45,
            BlockType::AcaciaPlanks => 46,
            BlockType::DarkOakPlanks => 47,
            BlockType::MangorvePlanks => 48,
            BlockType::CherryPlanks => 49,
            BlockType::BambooPlanks => 50,
            BlockType::CrimsonPlanks => 51,
            BlockType::WarpedPlanks => 52,
            BlockType::PaleOakPlanks => 53,
            BlockType::Cobblestone => 54,
            BlockType::MossyCobblestone => 55,
            BlockType::StoneBricks => 56,
            BlockType::ChiseledStoneBricks => 57,
            BlockType::MossyStoneBricks => 58,
            BlockType::SmoothStone => 59,
            BlockType::Sandstone => match face {
                FaceDirection::Top => 60,
                FaceDirection::Bottom => 61,
                _ => 59,
            },
            BlockType::ChiseledSandstone => match face {
                FaceDirection::Top => 60,
                FaceDirection::Bottom => 61,
                _ => 63,
            },
            BlockType::Terracotta => 64,
            BlockType::BlackTerracotta => 65,
            BlockType::BlueTerracotta => 66,
            BlockType::BrownTerracotta => 67,
            BlockType::CyanTerracotta => 68,
            BlockType::GrayTerracotta => 69,
            BlockType::GreenTerracotta => 70,
            BlockType::LightBlueTerracotta => 71,
            BlockType::LightGrayTerracotta => 72,
            BlockType::LimeTerracotta => 73,
            BlockType::MagentaTerracotta => 74,
            BlockType::OrangeTerracotta => 75,
            BlockType::PinkTerracotta => 76,
            BlockType::PurpleTerracotta => 77,
            BlockType::RedTerracotta => 78,
            BlockType::WhiteTerracotta => 79,
            BlockType::YellowTerracotta => 80,
            BlockType::BlackGlazedTerracotta => 81,
            BlockType::BlueGlazedTerracotta => 82,
            BlockType::BrownGlazedTerracotta => 83,
            BlockType::CyanGlazedTerracotta => 84,
            BlockType::GrayGlazedTerracotta => 85,
            BlockType::GreenGlazedTerracotta => 86,
            BlockType::LightBlueGlazedTerracotta => 87,
            BlockType::LightGrayGlazedTerracotta => 88,
            BlockType::LimeGlazedTerracotta => 89,
            BlockType::MagentaGlazedTerracotta => 90,
            BlockType::OrangeGlazedTerracotta => 91,
            BlockType::PinkGlazedTerracotta => 92,
            BlockType::PurpleGlazedTerracotta => 93,
            BlockType::RedGlazedTerracotta => 94,
            BlockType::WhiteGlazedTerracotta => 95,
            BlockType::YellowGlazedTerracotta => 96,
            BlockType::CopperBlock => 97,
            BlockType::DiamondBlock => 98,
            BlockType::EmeraldBlock => 99,
            BlockType::GoldBlock => 100,
            BlockType::IronBlock => 101,
            BlockType::LapisBlock => 102,
            BlockType::Air => 0,
        };
        TextureCoords::new(index, self.atlas_width, self.atlas_height)
    }

    pub fn get_tint(&self, block: BlockType) -> [f32; 3] {
        match block {
            BlockType::Grass => self.grass_color,
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
