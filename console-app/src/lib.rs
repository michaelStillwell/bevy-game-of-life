use gof_plugin::resources::{Board, Cell};

/// Demonstrates the Game of Life library for max 25 iterations
pub fn run() {
    // empty board
    // let mut board = Board::new(10, 10);
    // board.fill_at(0, 0);
    // board.fill_at(1, 1);
    // board.fill_at(0, 1);
    // board.fill_at(0, 1);

    // instantiated board
    let mut board = Board::from(vec![
        vec![
            Cell::Full,
            Cell::Full,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
        ],
        vec![
            Cell::Full,
            Cell::Full,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
        ],
        vec![
            Cell::Empty,
            Cell::Empty,
            Cell::Full,
            Cell::Empty,
            Cell::Empty,
        ],
        vec![
            Cell::Empty,
            Cell::Full,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
        ],
        vec![
            Cell::Empty,
            Cell::Empty,
            Cell::Full,
            Cell::Empty,
            Cell::Empty,
        ],
    ]);

    println!("base: {}", board.to_string());
    let mut i = 1;
    while board.next() && i <= 25 {
        println!("next x{i}: {}", board.to_string());
        i += 1;
    }
    println!("Finished");
}
