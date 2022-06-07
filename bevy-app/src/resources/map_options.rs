use bevy::prelude::*;

#[derive(Debug, Clone)]
pub enum CellSize {
    Fixed(f32),
    Adaptive { min: f32, max: f32 },
}

#[derive(Debug, Clone)]
pub enum MapPosition {
    Centered { offset: Vec3 },
    Custom(Vec3),
}

#[derive(Debug, Clone)]
pub struct MapOptions {
    pub map_size: (u8, u8),
    pub position: MapPosition,
    pub cell_size: CellSize,
    pub cell_padding: f32,
}

impl Default for MapPosition {
    fn default() -> Self {
        Self::Centered {
            offset: Default::default(),
        }
    }
}

impl Default for CellSize {
    fn default() -> Self {
        Self::Adaptive { min: 10., max: 50. }
    }
}

impl Default for MapOptions {
    fn default() -> Self {
        Self {
            map_size: (10, 10),
            position: Default::default(),
            cell_size: Default::default(),
            cell_padding: 0.,
        }
    }
}
