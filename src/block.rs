#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BlockType {
    Air,
    Grass,
    GrassSnowy,
    Dirt,
    Stone,
    Wood,
    Leaves,
    Water,
    Snow,
    SnowLayer,
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
            BlockType::Wood => [0.55, 0.35, 0.2],
            BlockType::Leaves => [0.1, 0.6, 0.1],
            BlockType::Water => [0.2, 0.4, 0.8],
            BlockType::Snow => [1.0, 1.0, 1.0],
            BlockType::SnowLayer => [1.0, 1.0, 1.0],
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
