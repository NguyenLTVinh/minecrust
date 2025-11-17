use std::collections::HashMap;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockRotation {
    None,
    Top,
    Side,
}

#[derive(Clone, Copy)]
pub enum TexturePattern {
    Uniform(usize),
    Custom([usize; 6]),
}

impl TexturePattern {
    pub fn get_texture(&self, face: FaceDirection) -> usize {
        match self {
            TexturePattern::Uniform(tex) => *tex,
            TexturePattern::Custom(textures) => textures[face as usize],
        }
    }
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

#[derive(Clone)]
pub struct Block {
    pub name: String,
    pub render_type: RenderType,
    pub dimensions: BlockDimensions,
    pub is_transparent: bool,
    pub texture: TexturePattern,
    pub rotation: BlockRotation,
}

impl Block {
    fn new(
        name: &str,
        render_type: RenderType,
        is_transparent: bool,
        texture: TexturePattern,
    ) -> Self {
        Block {
            name: name.to_string(),
            render_type,
            dimensions: BlockDimensions::full(),
            is_transparent,
            texture,
            rotation: BlockRotation::None,
        }
    }

    fn with_dimensions(mut self, dimensions: BlockDimensions) -> Self {
        self.dimensions = dimensions;
        self
    }

    fn with_rotation(mut self, rotation: BlockRotation) -> Self {
        self.rotation = rotation;
        self
    }
}

pub struct BlockRegistry {
    blocks: HashMap<BlockType, Block>,
}

impl BlockRegistry {
    pub fn new() -> Self {
        let mut blocks = HashMap::new();

        blocks.insert(
            BlockType::Stone,
            Block::new(
                "Stone",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(0),
            ),
        );
        blocks.insert(
            BlockType::Dirt,
            Block::new(
                "Dirt",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(1),
            ),
        );
        blocks.insert(
            BlockType::Sand,
            Block::new(
                "Sand",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(10),
            ),
        );
        blocks.insert(
            BlockType::Cobblestone,
            Block::new(
                "Cobblestone",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(54),
            ),
        );
        blocks.insert(
            BlockType::MossyCobblestone,
            Block::new(
                "MossyCobblestone",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(55),
            ),
        );
        blocks.insert(
            BlockType::StoneBricks,
            Block::new(
                "StoneBricks",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(56),
            ),
        );
        blocks.insert(
            BlockType::ChiseledStoneBricks,
            Block::new(
                "ChiseledStoneBricks",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(57),
            ),
        );
        blocks.insert(
            BlockType::MossyStoneBricks,
            Block::new(
                "MossyStoneBricks",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(58),
            ),
        );
        blocks.insert(
            BlockType::SmoothStone,
            Block::new(
                "SmoothStone",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(59),
            ),
        );
        blocks.insert(
            BlockType::CopperBlock,
            Block::new(
                "CopperBlock",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(97),
            ),
        );
        blocks.insert(
            BlockType::DiamondBlock,
            Block::new(
                "DiamondBlock",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(98),
            ),
        );
        blocks.insert(
            BlockType::EmeraldBlock,
            Block::new(
                "EmeraldBlock",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(99),
            ),
        );
        blocks.insert(
            BlockType::GoldBlock,
            Block::new(
                "GoldBlock",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(100),
            ),
        );
        blocks.insert(
            BlockType::IronBlock,
            Block::new(
                "IronBlock",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(101),
            ),
        );
        blocks.insert(
            BlockType::LapisBlock,
            Block::new(
                "LapisBlock",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(102),
            ),
        );
        blocks.insert(
            BlockType::Terracotta,
            Block::new(
                "Terracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(64),
            ),
        );

        blocks.insert(
            BlockType::GrassBlock,
            Block::new(
                "GrassBlock",
                RenderType::FullCube,
                false,
                TexturePattern::Custom([3, 1, 2, 2, 2, 2]),
            ),
        );
        blocks.insert(
            BlockType::SnowyGrassBlock,
            Block::new(
                "SnowyGrassBlock",
                RenderType::FullCube,
                false,
                TexturePattern::Custom([8, 1, 9, 9, 9, 9]),
            ),
        );
        blocks.insert(
            BlockType::Podzol,
            Block::new(
                "Podzol",
                RenderType::FullCube,
                false,
                TexturePattern::Custom([28, 1, 29, 29, 29, 29]),
            ),
        );

        blocks.insert(
            BlockType::OakLog,
            Block::new(
                "OakLog",
                RenderType::FullCube,
                false,
                TexturePattern::Custom([5, 5, 4, 4, 4, 4]),
            ),
        );
        blocks.insert(
            BlockType::SpruceLog,
            Block::new(
                "SpruceLog",
                RenderType::FullCube,
                false,
                TexturePattern::Custom([12, 12, 11, 11, 11, 11]),
            ),
        );
        blocks.insert(
            BlockType::BirchLog,
            Block::new(
                "BirchLog",
                RenderType::FullCube,
                false,
                TexturePattern::Custom([15, 15, 14, 14, 14, 14]),
            ),
        );
        blocks.insert(
            BlockType::JungleLog,
            Block::new(
                "JungleLog",
                RenderType::FullCube,
                false,
                TexturePattern::Custom([40, 40, 39, 39, 39, 39]),
            ),
        );

        blocks.insert(
            BlockType::OakLeaves,
            Block::new(
                "OakLeaves",
                RenderType::FullCube,
                true,
                TexturePattern::Uniform(6),
            ),
        );
        blocks.insert(
            BlockType::SpruceLeaves,
            Block::new(
                "SpruceLeaves",
                RenderType::FullCube,
                true,
                TexturePattern::Uniform(13),
            ),
        );
        blocks.insert(
            BlockType::BirchLeaves,
            Block::new(
                "BirchLeaves",
                RenderType::FullCube,
                true,
                TexturePattern::Uniform(16),
            ),
        );
        blocks.insert(
            BlockType::JungleLeaves,
            Block::new(
                "JungleLeaves",
                RenderType::FullCube,
                true,
                TexturePattern::Uniform(41),
            ),
        );

        blocks.insert(
            BlockType::OakPlanks,
            Block::new(
                "OakPlanks",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(42),
            ),
        );
        blocks.insert(
            BlockType::Spruceplanks,
            Block::new(
                "Spruceplanks",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(43),
            ),
        );
        blocks.insert(
            BlockType::BirchPlanks,
            Block::new(
                "BirchPlanks",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(44),
            ),
        );
        blocks.insert(
            BlockType::JunglePlanks,
            Block::new(
                "JunglePlanks",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(45),
            ),
        );
        blocks.insert(
            BlockType::AcaciaPlanks,
            Block::new(
                "AcaciaPlanks",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(46),
            ),
        );
        blocks.insert(
            BlockType::DarkOakPlanks,
            Block::new(
                "DarkOakPlanks",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(47),
            ),
        );
        blocks.insert(
            BlockType::MangorvePlanks,
            Block::new(
                "MangorvePlanks",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(48),
            ),
        );
        blocks.insert(
            BlockType::CherryPlanks,
            Block::new(
                "CherryPlanks",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(49),
            ),
        );
        blocks.insert(
            BlockType::BambooPlanks,
            Block::new(
                "BambooPlanks",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(50),
            ),
        );
        blocks.insert(
            BlockType::CrimsonPlanks,
            Block::new(
                "CrimsonPlanks",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(51),
            ),
        );
        blocks.insert(
            BlockType::WarpedPlanks,
            Block::new(
                "WarpedPlanks",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(52),
            ),
        );
        blocks.insert(
            BlockType::PaleOakPlanks,
            Block::new(
                "PaleOakPlanks",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(53),
            ),
        );

        blocks.insert(
            BlockType::Sandstone,
            Block::new(
                "Sandstone",
                RenderType::FullCube,
                false,
                TexturePattern::Custom([61, 62, 60, 60, 60, 60]),
            ),
        );
        blocks.insert(
            BlockType::ChiseledSandstone,
            Block::new(
                "ChiseledSandstone",
                RenderType::FullCube,
                false,
                TexturePattern::Custom([61, 62, 63, 63, 63, 63]),
            ),
        );

        blocks.insert(
            BlockType::BlackTerracotta,
            Block::new(
                "BlackTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(65),
            ),
        );
        blocks.insert(
            BlockType::BlueTerracotta,
            Block::new(
                "BlueTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(66),
            ),
        );
        blocks.insert(
            BlockType::BrownTerracotta,
            Block::new(
                "BrownTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(67),
            ),
        );
        blocks.insert(
            BlockType::CyanTerracotta,
            Block::new(
                "CyanTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(68),
            ),
        );
        blocks.insert(
            BlockType::GrayTerracotta,
            Block::new(
                "GrayTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(69),
            ),
        );
        blocks.insert(
            BlockType::GreenTerracotta,
            Block::new(
                "GreenTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(70),
            ),
        );
        blocks.insert(
            BlockType::LightBlueTerracotta,
            Block::new(
                "LightBlueTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(71),
            ),
        );
        blocks.insert(
            BlockType::LightGrayTerracotta,
            Block::new(
                "LightGrayTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(72),
            ),
        );
        blocks.insert(
            BlockType::LimeTerracotta,
            Block::new(
                "LimeTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(73),
            ),
        );
        blocks.insert(
            BlockType::MagentaTerracotta,
            Block::new(
                "MagentaTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(74),
            ),
        );
        blocks.insert(
            BlockType::OrangeTerracotta,
            Block::new(
                "OrangeTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(75),
            ),
        );
        blocks.insert(
            BlockType::PinkTerracotta,
            Block::new(
                "PinkTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(76),
            ),
        );
        blocks.insert(
            BlockType::PurpleTerracotta,
            Block::new(
                "PurpleTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(77),
            ),
        );
        blocks.insert(
            BlockType::RedTerracotta,
            Block::new(
                "RedTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(78),
            ),
        );
        blocks.insert(
            BlockType::WhiteTerracotta,
            Block::new(
                "WhiteTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(79),
            ),
        );
        blocks.insert(
            BlockType::YellowTerracotta,
            Block::new(
                "YellowTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(80),
            ),
        );

        blocks.insert(
            BlockType::BlackGlazedTerracotta,
            Block::new(
                "BlackGlazedTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(81),
            ),
        );
        blocks.insert(
            BlockType::BlueGlazedTerracotta,
            Block::new(
                "BlueGlazedTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(82),
            ),
        );
        blocks.insert(
            BlockType::BrownGlazedTerracotta,
            Block::new(
                "BrownGlazedTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(83),
            ),
        );
        blocks.insert(
            BlockType::CyanGlazedTerracotta,
            Block::new(
                "CyanGlazedTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(84),
            ),
        );
        blocks.insert(
            BlockType::GrayGlazedTerracotta,
            Block::new(
                "GrayGlazedTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(85),
            ),
        );
        blocks.insert(
            BlockType::GreenGlazedTerracotta,
            Block::new(
                "GreenGlazedTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(86),
            ),
        );
        blocks.insert(
            BlockType::LightBlueGlazedTerracotta,
            Block::new(
                "LightBlueGlazedTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(87),
            ),
        );
        blocks.insert(
            BlockType::LightGrayGlazedTerracotta,
            Block::new(
                "LightGrayGlazedTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(88),
            ),
        );
        blocks.insert(
            BlockType::LimeGlazedTerracotta,
            Block::new(
                "LimeGlazedTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(89),
            ),
        );
        blocks.insert(
            BlockType::MagentaGlazedTerracotta,
            Block::new(
                "MagentaGlazedTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(90),
            ),
        );
        blocks.insert(
            BlockType::OrangeGlazedTerracotta,
            Block::new(
                "OrangeGlazedTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(91),
            ),
        );
        blocks.insert(
            BlockType::PinkGlazedTerracotta,
            Block::new(
                "PinkGlazedTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(92),
            ),
        );
        blocks.insert(
            BlockType::PurpleGlazedTerracotta,
            Block::new(
                "PurpleGlazedTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(93),
            ),
        );
        blocks.insert(
            BlockType::RedGlazedTerracotta,
            Block::new(
                "RedGlazedTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(94),
            ),
        );
        blocks.insert(
            BlockType::WhiteGlazedTerracotta,
            Block::new(
                "WhiteGlazedTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(95),
            ),
        );
        blocks.insert(
            BlockType::YellowGlazedTerracotta,
            Block::new(
                "YellowGlazedTerracotta",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(96),
            ),
        );

        blocks.insert(
            BlockType::Water,
            Block::new(
                "Water",
                RenderType::FullCube,
                true,
                TexturePattern::Uniform(7),
            ),
        );
        blocks.insert(
            BlockType::Snow,
            Block::new(
                "Snow",
                RenderType::FullCube,
                false,
                TexturePattern::Uniform(8),
            ),
        );
        blocks.insert(
            BlockType::SnowLayer,
            Block::new(
                "SnowLayer",
                RenderType::ScaledCube,
                true,
                TexturePattern::Uniform(8),
            )
            .with_dimensions(BlockDimensions::from_pixels(16, 2, 16)),
        );

        blocks.insert(
            BlockType::BrownMushroom,
            Block::new(
                "BrownMushroom",
                RenderType::CrossPlant,
                true,
                TexturePattern::Uniform(17),
            ),
        );
        blocks.insert(
            BlockType::RedMushroom,
            Block::new(
                "RedMushroom",
                RenderType::CrossPlant,
                true,
                TexturePattern::Uniform(22),
            ),
        );
        blocks.insert(
            BlockType::Poppy,
            Block::new(
                "Poppy",
                RenderType::CrossPlant,
                true,
                TexturePattern::Uniform(18),
            ),
        );
        blocks.insert(
            BlockType::PinkTulip,
            Block::new(
                "PinkTulip",
                RenderType::CrossPlant,
                true,
                TexturePattern::Uniform(25),
            ),
        );
        blocks.insert(
            BlockType::TorchFlower,
            Block::new(
                "TorchFlower",
                RenderType::CrossPlant,
                true,
                TexturePattern::Uniform(24),
            ),
        );
        blocks.insert(
            BlockType::ShortGrass,
            Block::new(
                "ShortGrass",
                RenderType::CrossPlant,
                true,
                TexturePattern::Uniform(19),
            ),
        );
        blocks.insert(
            BlockType::TallGrassTop,
            Block::new(
                "TallGrassTop",
                RenderType::CrossPlant,
                true,
                TexturePattern::Uniform(20),
            ),
        );
        blocks.insert(
            BlockType::TallGrassBottom,
            Block::new(
                "TallGrassBottom",
                RenderType::CrossPlant,
                true,
                TexturePattern::Uniform(27),
            ),
        );
        blocks.insert(
            BlockType::DeadBush,
            Block::new(
                "DeadBush",
                RenderType::CrossPlant,
                true,
                TexturePattern::Uniform(21),
            ),
        );
        blocks.insert(
            BlockType::TallDryGrass,
            Block::new(
                "TallDryGrass",
                RenderType::CrossPlant,
                true,
                TexturePattern::Uniform(23),
            ),
        );
        blocks.insert(
            BlockType::ShortDryGrass,
            Block::new(
                "ShortDryGrass",
                RenderType::CrossPlant,
                true,
                TexturePattern::Uniform(26),
            ),
        );
        blocks.insert(
            BlockType::Fern,
            Block::new(
                "Fern",
                RenderType::CrossPlant,
                true,
                TexturePattern::Uniform(30),
            ),
        );
        blocks.insert(
            BlockType::LargeFernTop,
            Block::new(
                "LargeFernTop",
                RenderType::CrossPlant,
                true,
                TexturePattern::Uniform(31),
            ),
        );
        blocks.insert(
            BlockType::LargeFernBottom,
            Block::new(
                "LargeFernBottom",
                RenderType::CrossPlant,
                true,
                TexturePattern::Uniform(32),
            ),
        );
        blocks.insert(
            BlockType::SweetBerryBushStage1,
            Block::new(
                "SweetBerryBushStage1",
                RenderType::CrossPlant,
                true,
                TexturePattern::Uniform(33),
            ),
        );
        blocks.insert(
            BlockType::SweetBerryBushStage2,
            Block::new(
                "SweetBerryBushStage2",
                RenderType::CrossPlant,
                true,
                TexturePattern::Uniform(34),
            ),
        );
        blocks.insert(
            BlockType::CactusFlower,
            Block::new(
                "CactusFlower",
                RenderType::CrossPlant,
                true,
                TexturePattern::Uniform(38),
            ),
        );

        blocks.insert(
            BlockType::Cactus,
            Block::new(
                "Cactus",
                RenderType::ScaledCube,
                true,
                TexturePattern::Custom([36, 37, 35, 35, 35, 35]),
            )
            .with_dimensions(BlockDimensions::from_pixels(14, 16, 14)),
        );

        blocks.insert(
            BlockType::Air,
            Block::new(
                "Air",
                RenderType::FullCube,
                true,
                TexturePattern::Uniform(0),
            ),
        );

        BlockRegistry { blocks }
    }

    pub fn get_block(&self, block_type: BlockType) -> Option<&Block> {
        self.blocks.get(&block_type)
    }

    pub fn is_transparent(&self, block_type: BlockType) -> bool {
        self.get_block(block_type)
            .map(|b| b.is_transparent)
            .unwrap_or(true)
    }
}

#[derive(Clone, Copy)]
pub struct BlockProperties {
    pub render_type: RenderType,
    pub dimensions: BlockDimensions,
}

impl BlockType {
    pub fn get_block(&self) -> Option<&'static Block> {
        BLOCK_REGISTRY.get_block(*self)
    }

    pub fn get_properties(&self) -> BlockProperties {
        BlockProperties {
            render_type: self.get_render_type(),
            dimensions: self.get_dimensions(),
        }
    }

    pub fn get_render_type(&self) -> RenderType {
        self.get_block()
            .map(|b| b.render_type)
            .unwrap_or(RenderType::FullCube)
    }

    pub fn get_dimensions(&self) -> BlockDimensions {
        self.get_block()
            .map(|b| b.dimensions)
            .unwrap_or(BlockDimensions::full())
    }

    pub fn is_transparent(&self) -> bool {
        BLOCK_REGISTRY.is_transparent(*self)
    }

    pub fn is_full_block(&self) -> bool {
        self.get_render_type() == RenderType::FullCube
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
