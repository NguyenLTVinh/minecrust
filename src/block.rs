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
}

impl BlockType {
    pub fn is_solid(&self) -> bool {
        !matches!(
            self,
            BlockType::Air | BlockType::Water | BlockType::SnowLayer
        )
    }

    pub fn is_transparent(&self) -> bool {
        matches!(
            self,
            BlockType::Air | BlockType::Water | BlockType::SnowLayer
        )
    }

    pub fn get_color(&self) -> [f32; 3] {
        match self {
            BlockType::Air => [0.0, 0.0, 0.0],
            BlockType::Grass => [0.2, 0.8, 0.2],
            BlockType::GrassSnowy => [0.2, 0.8, 0.2],
            BlockType::Dirt => [0.6, 0.4, 0.2],
            BlockType::Stone => [0.5, 0.5, 0.5],
            BlockType::Sand => [0.9, 0.8, 0.6],
            BlockType::OakLog => [0.55, 0.35, 0.2],
            BlockType::OakLeaves => [0.1, 0.6, 0.1],
            BlockType::Water => [0.2, 0.4, 0.8],
            BlockType::Snow => [1.0, 1.0, 1.0],
            BlockType::SnowLayer => [1.0, 1.0, 1.0],
            BlockType::SpruceLog => [0.4, 0.25, 0.15],
            BlockType::SpruceLeaves => [0.2, 0.4, 0.2],
            BlockType::BirchLog => [0.9, 0.9, 0.85],
            BlockType::BirchLeaves => [0.1, 0.6, 0.1],
        }
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
