#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BlockType {
    Air,
    Grass,
    GrassSnowy,
    Dirt,
    Stone,
    Sand,
    OakLog,
    OakLeaves,
    Water,
    Snow,
    SnowLayer,
    SpruceLog,
    SpruceLeaves,
    BirchLog,
    BirchLeaves,
    BrownMushroom,
    Poppy,
    ShortGrass,
    TallGrassTop,
    DeadBush,
    RedMushroom,
    TallDryGrass,
    TorchFlower,
    PinkTulip,
    ShortDryGrass,
    TallGrassBottom,
    Podzol,
    Fern,
    LargeFernTop,
    LargeFernBottom,
    SweetBerryBushStage1,
    SweetBerryBushStage2,
    Cactus,
    CactusFlower,
}

#[derive(Clone, Copy)]
pub struct BlockProperties {
    pub is_transparent: bool,
    pub is_decorative: bool,
    pub is_full_block: bool,
}

pub struct BlockRegistry {
    properties: std::collections::HashMap<BlockType, BlockProperties>,
}

impl BlockRegistry {
    pub fn new() -> Self {
        let mut properties = std::collections::HashMap::new();

        let decorative_blocks = [
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
            BlockType::Fern,
            BlockType::LargeFernTop,
            BlockType::LargeFernBottom,
            BlockType::SweetBerryBushStage1,
            BlockType::SweetBerryBushStage2,
            BlockType::CactusFlower,
        ];

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
            BlockType::Fern,
            BlockType::LargeFernTop,
            BlockType::LargeFernBottom,
            BlockType::SweetBerryBushStage1,
            BlockType::SweetBerryBushStage2,
            BlockType::CactusFlower,
        ];

        let non_full_blocks = [
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
            BlockType::Fern,
            BlockType::LargeFernTop,
            BlockType::LargeFernBottom,
            BlockType::SweetBerryBushStage1,
            BlockType::SweetBerryBushStage2,
            BlockType::SnowLayer,
            BlockType::CactusFlower,
        ];

        let all_blocks = [
            BlockType::Air,
            BlockType::Grass,
            BlockType::GrassSnowy,
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
        ];

        for block in all_blocks.iter() {
            let is_decorative = decorative_blocks.contains(block);
            let is_transparent = transparent_blocks.contains(block);
            let is_full_block = !non_full_blocks.contains(block);

            properties.insert(
                *block,
                BlockProperties {
                    is_transparent,
                    is_decorative,
                    is_full_block,
                },
            );
        }

        BlockRegistry { properties }
    }

    pub fn get_properties(&self, block: BlockType) -> BlockProperties {
        self.properties
            .get(&block)
            .copied()
            .unwrap_or(BlockProperties {
                is_transparent: true,
                is_decorative: false,
                is_full_block: false,
            })
    }

    pub fn is_transparent(&self, block: BlockType) -> bool {
        self.get_properties(block).is_transparent
    }

    pub fn is_decorative(&self, block: BlockType) -> bool {
        self.get_properties(block).is_decorative
    }

    pub fn is_full_block(&self, block: BlockType) -> bool {
        self.get_properties(block).is_full_block
    }
}

impl BlockType {
    pub fn is_decorative(&self) -> bool {
        BLOCK_REGISTRY.is_decorative(*self)
    }

    pub fn is_full_block(&self) -> bool {
        BLOCK_REGISTRY.is_full_block(*self)
    }

    pub fn is_transparent(&self) -> bool {
        BLOCK_REGISTRY.is_transparent(*self)
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
