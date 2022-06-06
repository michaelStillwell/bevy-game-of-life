use crate::resources::Cell;
use crate::utilities::Vec2;

const NEIGHBORS: [(i8, i8); 8] = [
    // (x, y)

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

// Board that houses all of the cells for the game
#[derive(Debug, Clone)]
pub struct Board {
    height: u8,
    width: u8,
    // all of the cells, false -> empty, true -> filled
    map: Vec<Vec<Cell>>,
}

impl Board {
    // Instantiate new board with all empty cells of given size
    pub fn new(height: u8, width: u8) -> Self {
        Self {
            map: Self::new_map(height, width),
            height,
            width,
        }
    }

    // Instantiate new board from a given 2d Vec
    pub fn from(map: Vec<Vec<Cell>>) -> Self {
        // TODO: this is vulnerable to crashing or weird behavior if not given rectangle vector
        let height = map.len() as u8;
        let width = map.iter().map(|v| v.len()).max().unwrap() as u8;
        Self { map, height, width }
    }

    // Converts board to a string
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
        let mut map = Self::new_map(self.height, self.width);
        let mut changed = false;

        for (y, col) in self.map.iter().enumerate() {
            for (x, row) in col.iter().enumerate() {
                map[y][x] = match row {
                    Cell::Empty => {
                        if self.will_spawn(x, y) {
                            changed = true;
                            Cell::Full
                        } else {
                            Cell::Empty
                        }
                    }
                    Cell::Full => {
                        if self.will_die(x, y) {
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
        let mut map = Self::new_map(self.height, self.width);

        for (y, col) in self.map.iter().enumerate() {
            for (x, _row) in col.iter().enumerate() {
                map[y][x] = Cell::Empty;
            }
        }
        self.map = map;
    }

    pub fn fill_at(&mut self, x: usize, y: usize) {
        if self.in_map(Vec2::new(x as i8, y as i8)) {
            self.map[y][x] = Cell::Full;
        }
    }

    fn new_map(height: u8, width: u8) -> Vec<Vec<Cell>> {
        (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Cell::Empty).collect())
            .collect()
    }

    fn will_die(&self, x: usize, y: usize) -> bool {
        let neighbor_count = self.get_neighbors(x, y);
        //  if four or more neighbors exist OR if one or less neighbors exist
        neighbor_count >= 4 || neighbor_count <= 1
    }

    fn will_spawn(&self, x: usize, y: usize) -> bool {
        // if three neighbors exist
        self.get_neighbors(x, y) == 3
    }

    fn get_neighbors(&self, x: usize, y: usize) -> u8 {
        // NOTE: I changed this because I like it better, might not be faster though
        NEIGHBORS
            .iter()
            // .copied() // not sure why you need to copy?
            .map(|(nx, ny)| (x as i8 + nx, y as i8 + ny))
            .filter(|(x, y)| {
                self.in_map(Vec2::new(*x, *y)) && !self.map[*y as usize][*x as usize].is_empty()
            })
            .count() as u8
    }

    fn in_map(&self, position: Vec2) -> bool {
        position.x >= 0
            && position.y >= 0
            && position.x < self.width as i8
            && position.y < self.height as i8
    }
}
