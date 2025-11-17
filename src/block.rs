#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Debug,
    strum_macros::EnumString,
    strum_macros::IntoStaticStr,
    strum_macros::EnumIter,
)]
pub enum BlockType {
    GrassBlock,
    SnowyGrassBlock,
    Dirt,
    Podzol,
    Stone,
    Sand,
    OakLog,
    SpruceLog,
    BirchLog,
    JungleLog,
    OakLeaves,
    SpruceLeaves,
    BirchLeaves,
    JungleLeaves,
    OakPlanks,
    Spruceplanks,
    BirchPlanks,
    JunglePlanks,
    AcaciaPlanks,
    DarkOakPlanks,
    MangorvePlanks,
    CherryPlanks,
    BambooPlanks,
    CrimsonPlanks,
    WarpedPlanks,
    PaleOakPlanks,
    Cobblestone,
    MossyCobblestone,
    StoneBricks,
    ChiseledStoneBricks,
    MossyStoneBricks,
    SmoothStone,
    Sandstone,
    ChiseledSandstone,
    CopperBlock,
    DiamondBlock,
    EmeraldBlock,
    GoldBlock,
    IronBlock,
    LapisBlock,
    Terracotta,
    BlackTerracotta,
    BlueTerracotta,
    BrownTerracotta,
    CyanTerracotta,
    GrayTerracotta,
    GreenTerracotta,
    LightBlueTerracotta,
    LightGrayTerracotta,
    LimeTerracotta,
    MagentaTerracotta,
    OrangeTerracotta,
    PinkTerracotta,
    PurpleTerracotta,
    RedTerracotta,
    WhiteTerracotta,
    YellowTerracotta,
    BlackGlazedTerracotta,
    BlueGlazedTerracotta,
    BrownGlazedTerracotta,
    CyanGlazedTerracotta,
    GrayGlazedTerracotta,
    GreenGlazedTerracotta,
    LightBlueGlazedTerracotta,
    LightGrayGlazedTerracotta,
    LimeGlazedTerracotta,
    MagentaGlazedTerracotta,
    OrangeGlazedTerracotta,
    PinkGlazedTerracotta,
    PurpleGlazedTerracotta,
    RedGlazedTerracotta,
    WhiteGlazedTerracotta,
    YellowGlazedTerracotta,
    Water,
    Snow,
    SnowLayer,
    BrownMushroom,
    RedMushroom,
    Poppy,
    PinkTulip,
    TorchFlower,
    ShortGrass,
    TallGrassTop,
    TallGrassBottom,
    TallDryGrass,
    ShortDryGrass,
    DeadBush,
    Fern,
    LargeFernTop,
    LargeFernBottom,
    Cactus,
    CactusFlower,
    SweetBerryBushStage1,
    SweetBerryBushStage2,
    Air,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderType {
    FullCube,
    ScaledCube,
    CrossPlant,
}

#[derive(Clone, Copy)]
pub struct BlockDimensions {
    pub width_pixels: u32,
    pub height_pixels: u32,
    pub length_pixels: u32,
}

impl BlockDimensions {
    pub fn full() -> Self {
        BlockDimensions {
            width_pixels: 16,
            height_pixels: 16,
            length_pixels: 16,
        }
    }

    pub fn from_pixels(width: u32, height: u32, length: u32) -> Self {
        BlockDimensions {
            width_pixels: width,
            height_pixels: height,
            length_pixels: length,
        }
    }
}

#[derive(Clone, Copy)]
pub struct BlockProperties {
    pub is_transparent: bool,
    pub render_type: RenderType,
    pub dimensions: BlockDimensions,
}

pub struct BlockRegistry {
    properties: std::collections::HashMap<BlockType, BlockProperties>,
}

impl BlockRegistry {
    pub fn new() -> Self {
        let mut properties = std::collections::HashMap::new();

        let transparent_blocks = [
            BlockType::Air,
            BlockType::Water,
            BlockType::SnowLayer,
            BlockType::BrownMushroom,
            BlockType::Poppy,
            BlockType::ShortGrass,
            BlockType::TallGrassTop,
            BlockType::DeadBush,
            BlockType::RedMushroom,
            BlockType::TallDryGrass,
            BlockType::TorchFlower,
            BlockType::PinkTulip,
            BlockType::ShortDryGrass,
            BlockType::TallGrassBottom,
            BlockType::BirchLeaves,
            BlockType::OakLeaves,
            BlockType::SpruceLeaves,
            BlockType::JungleLeaves,
            BlockType::Fern,
            BlockType::LargeFernTop,
            BlockType::LargeFernBottom,
            BlockType::SweetBerryBushStage1,
            BlockType::SweetBerryBushStage2,
            BlockType::CactusFlower,
            BlockType::Cactus,
        ];

        let all_blocks = [
            BlockType::Air,
            BlockType::GrassBlock,
            BlockType::SnowyGrassBlock,
            BlockType::Dirt,
            BlockType::Stone,
            BlockType::Sand,
            BlockType::OakLog,
            BlockType::OakLeaves,
            BlockType::Water,
            BlockType::Snow,
            BlockType::SnowLayer,
            BlockType::SpruceLog,
            BlockType::SpruceLeaves,
            BlockType::BirchLog,
            BlockType::BirchLeaves,
            BlockType::BrownMushroom,
            BlockType::Poppy,
            BlockType::ShortGrass,
            BlockType::TallGrassTop,
            BlockType::DeadBush,
            BlockType::RedMushroom,
            BlockType::TallDryGrass,
            BlockType::TorchFlower,
            BlockType::PinkTulip,
            BlockType::ShortDryGrass,
            BlockType::TallGrassBottom,
            BlockType::Podzol,
            BlockType::Fern,
            BlockType::LargeFernTop,
            BlockType::LargeFernBottom,
            BlockType::SweetBerryBushStage1,
            BlockType::SweetBerryBushStage2,
            BlockType::Cactus,
            BlockType::CactusFlower,
            BlockType::JungleLog,
            BlockType::JungleLeaves,
            BlockType::OakPlanks,
            BlockType::Spruceplanks,
            BlockType::BirchPlanks,
            BlockType::JunglePlanks,
            BlockType::AcaciaPlanks,
            BlockType::DarkOakPlanks,
            BlockType::MangorvePlanks,
            BlockType::CherryPlanks,
            BlockType::BambooPlanks,
            BlockType::CrimsonPlanks,
            BlockType::WarpedPlanks,
            BlockType::PaleOakPlanks,
            BlockType::Cobblestone,
            BlockType::MossyCobblestone,
            BlockType::StoneBricks,
            BlockType::ChiseledStoneBricks,
            BlockType::MossyStoneBricks,
            BlockType::SmoothStone,
            BlockType::Sandstone,
            BlockType::ChiseledSandstone,
            BlockType::Terracotta,
            BlockType::BlackTerracotta,
            BlockType::BlueTerracotta,
            BlockType::BrownTerracotta,
            BlockType::CyanTerracotta,
            BlockType::GrayTerracotta,
            BlockType::GreenTerracotta,
            BlockType::LightBlueTerracotta,
            BlockType::LightGrayTerracotta,
            BlockType::LimeTerracotta,
            BlockType::MagentaTerracotta,
            BlockType::OrangeTerracotta,
            BlockType::PinkTerracotta,
            BlockType::PurpleTerracotta,
            BlockType::RedTerracotta,
            BlockType::WhiteTerracotta,
            BlockType::YellowTerracotta,
            BlockType::BlackGlazedTerracotta,
            BlockType::BlueGlazedTerracotta,
            BlockType::BrownGlazedTerracotta,
            BlockType::CyanGlazedTerracotta,
            BlockType::GrayGlazedTerracotta,
            BlockType::GreenGlazedTerracotta,
            BlockType::LightBlueGlazedTerracotta,
            BlockType::LightGrayGlazedTerracotta,
            BlockType::LimeGlazedTerracotta,
            BlockType::MagentaGlazedTerracotta,
            BlockType::OrangeGlazedTerracotta,
            BlockType::PinkGlazedTerracotta,
            BlockType::PurpleGlazedTerracotta,
            BlockType::RedGlazedTerracotta,
            BlockType::WhiteGlazedTerracotta,
            BlockType::YellowGlazedTerracotta,
            BlockType::CopperBlock,
            BlockType::DiamondBlock,
            BlockType::EmeraldBlock,
            BlockType::GoldBlock,
            BlockType::IronBlock,
            BlockType::LapisBlock,
        ];

        for block in all_blocks.iter() {
            let is_transparent = transparent_blocks.contains(block);
            let (render_type, dimensions) = Self::get_render_config(*block);

            properties.insert(
                *block,
                BlockProperties {
                    is_transparent,
                    render_type,
                    dimensions,
                },
            );
        }

        BlockRegistry { properties }
    }

    fn get_render_config(block: BlockType) -> (RenderType, BlockDimensions) {
        match block {
            BlockType::SnowLayer => (
                RenderType::ScaledCube,
                BlockDimensions::from_pixels(16, 2, 16),
            ),
            BlockType::Cactus => (
                RenderType::ScaledCube,
                BlockDimensions::from_pixels(14, 16, 14),
            ),
            BlockType::BrownMushroom
            | BlockType::Poppy
            | BlockType::ShortGrass
            | BlockType::TallGrassTop
            | BlockType::DeadBush
            | BlockType::RedMushroom
            | BlockType::TallDryGrass
            | BlockType::TorchFlower
            | BlockType::PinkTulip
            | BlockType::ShortDryGrass
            | BlockType::TallGrassBottom
            | BlockType::Fern
            | BlockType::LargeFernTop
            | BlockType::LargeFernBottom
            | BlockType::SweetBerryBushStage1
            | BlockType::SweetBerryBushStage2
            | BlockType::CactusFlower => (RenderType::CrossPlant, BlockDimensions::full()),
            _ => (RenderType::FullCube, BlockDimensions::full()),
        }
    }

    pub fn get_properties(&self, block: BlockType) -> BlockProperties {
        self.properties
            .get(&block)
            .copied()
            .unwrap_or(BlockProperties {
                is_transparent: true,
                render_type: RenderType::FullCube,
                dimensions: BlockDimensions::full(),
            })
    }

    pub fn is_transparent(&self, block: BlockType) -> bool {
        self.get_properties(block).is_transparent
    }
}

impl BlockType {
    pub fn get_properties(&self) -> BlockProperties {
        BLOCK_REGISTRY.get_properties(*self)
    }

    pub fn is_transparent(&self) -> bool {
        BLOCK_REGISTRY.is_transparent(*self)
    }

    pub fn is_full_block(&self) -> bool {
        self.get_properties().render_type == RenderType::FullCube
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FaceDirection {
    Top,
    Bottom,
    Front,
    Back,
    Right,
    Left,
}

lazy_static::lazy_static! {
    pub static ref BLOCK_REGISTRY: BlockRegistry = BlockRegistry::new();
}
