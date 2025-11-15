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
}

impl BlockType {
    pub fn is_decorative(&self) -> bool {
        matches!(
            self,
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
        )
    }

    pub fn is_full_block(&self) -> bool {
        !matches!(
            self,
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
                | BlockType::SnowLayer
        )
    }

    pub fn is_transparent(&self) -> bool {
        matches!(
            self,
            BlockType::Air
                | BlockType::Water
                | BlockType::SnowLayer
                | BlockType::BrownMushroom
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
                | BlockType::BirchLeaves
                | BlockType::OakLeaves
                | BlockType::SpruceLeaves
                | BlockType::Fern
                | BlockType::LargeFernTop
                | BlockType::LargeFernBottom
                | BlockType::SweetBerryBushStage1
                | BlockType::SweetBerryBushStage2
        )
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
