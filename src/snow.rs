/// Snow layer data - can stack from 1 to 8 layers (8 layers = 1 full block)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SnowLayer {
    /// Number of snow layers stacked (1-8)
    pub layers: u8,
}

impl SnowLayer {
    /// Create a new snow layer with the specified count (clamped to 1-8)
    pub fn new(layers: u8) -> Self {
        SnowLayer {
            layers: layers.clamp(1, 8),
        }
    }

    /// Get the height of this snow layer as a fraction of a full block
    pub fn get_height(&self) -> f32 {
        (self.layers as f32) / 8.0
    }

    /// Check if we can add more layers (max 8)
    pub fn can_add_layer(&self) -> bool {
        self.layers < 8
    }

    /// Add a layer and return the new snow layer (or None if already at max)
    pub fn add_layer(self) -> Option<Self> {
        if self.can_add_layer() {
            Some(SnowLayer::new(self.layers + 1))
        } else {
            None
        }
    }

    /// Try to add multiple layers at once
    pub fn add_layers(self, count: u8) -> Self {
        SnowLayer::new(self.layers + count)
    }
}

impl Default for SnowLayer {
    fn default() -> Self {
        SnowLayer::new(1)
    }
}
