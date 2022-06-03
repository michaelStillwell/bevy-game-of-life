use crate::resources::Cell;
use crate::utilities::Vec2;

/// Creates new map vector
pub fn new_map(height: u8, width: u8) -> Vec<Vec<Cell>> {
    (0..height)
        .into_iter()
        .map(|_| (0..width).into_iter().map(|_| Cell::Empty).collect())
        .collect()
}

/// check if position is in map
pub fn in_map(map: &Vec<Vec<Cell>>, position: Vec2) -> bool {
    let size = (map.len(), map[0].len());
    position.x >= 0 && position.y >= 0 && position.x < size.0 as i8 && position.y < size.1 as i8
}
