use crate::resources::Cell;
use crate::utilities::{in_map, new_map, Bounds2, Vec2};

const NEIGHBORS: [(i8, i8); 8] = [
    /// (x, y)
    // top left
    (-1, -1),
    // top
    (0, -1),
    // top right
    (1, -1),
    // right
    (1, 0),
    // left
    (-1, 0),
    // bottom left
    (-1, 1),
    // bottom
    (0, 1),
    // bottom right
    (1, 1),
];

/// Board that houses all of the cells for the game
#[derive(Debug, Clone)]
pub struct Board {
    height: u8,
    width: u8,
    /// all of the cells, false -> empty, true -> filled
    map: Vec<Vec<Cell>>,
}

impl Board {
    /// Instantiate new board with all empty cells of given size
    pub fn new(height: u8, width: u8) -> Self {
        Self {
            map: new_map(height, width),
            height,
            width,
        }
    }

    /// Instantiate new board from a given 2d Vec
    pub fn from(map: Vec<Vec<Cell>>) -> Self {
        // TODO: this is vulnerable to crashing or weird behavior if not given rectangle vector
        let height = map.len() as u8;
        let width = map[0].len() as u8;
        Self { map, height, width }
    }

    /// Converts board to a string
    pub fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str("[\n");
        for col in self.map.iter() {
            let row: Vec<String> = col.iter().map(|c| c.to_string()).collect();
            output.push_str(&format!(" [{}],\n", &row.join(", ")));
        }
        output.push_str("]\n");
        format!("{}", output)
    }

    pub fn next(&mut self) -> bool {
        let mut map = new_map(self.height, self.width);
        let mut changed = false;

        for (y, col) in self.map.iter().enumerate() {
            for (x, row) in col.iter().enumerate() {
                map[y][x] = match self.map[y][x] {
                    Cell::Empty => {
                        if self.will_spawn(x as u8, y as u8) {
                            changed = true;
                            Cell::Full
                        } else {
                            Cell::Empty
                        }
                    }
                    Cell::Full => {
                        if self.will_die(x as u8, y as u8) {
                            changed = true;
                            Cell::Empty
                        } else {
                            Cell::Full
                        }
                    }
                };
            }
        }
        if !changed {
            return false;
        }
        self.map = map;
        true
    }

    pub fn empty_map(&mut self) {
        let mut map = new_map(self.height, self.width);

        for (y, col) in self.map.iter().enumerate() {
            for (x, row) in col.iter().enumerate() {
                map[y][x] = Cell::Empty;
            }
        }
        self.map = map;
    }

    pub fn fill_at(&mut self, x: usize, y: usize) {
        if in_map(&self.map, Vec2::new(x as i8, y as i8)) {
            self.map[y][x] = Cell::Full;
        }
    }

    fn will_die(&self, x: u8, y: u8) -> bool {
        let neighbor_count = self.get_neighbors(x as usize, y as usize);
        //  if four or more neighbors exist OR if one or less neighbors exist
        neighbor_count >= 4 || neighbor_count <= 1
    }

    fn will_spawn(&self, x: u8, y: u8) -> bool {
        // if three neighbors exist
        self.get_neighbors(x as usize, y as usize) == 3
    }

    fn get_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut neighbor_count = 0u8;
        for (nx, ny) in NEIGHBORS {
            let position = Vec2::new(x as i8 + nx, y as i8 + ny);
            if in_map(&self.map, Vec2::new(position.x, position.y))
                && !self.map[position.y as usize][position.x as usize].is_empty()
            {
                neighbor_count += 1;
            }
        }
        neighbor_count
    }
}
