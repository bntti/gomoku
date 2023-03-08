use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    X,
    O,
    None,
}

pub struct GameState {
    pub x_turn: bool,
    pub board: [[Tile; 15]; 15],
}

impl GameState {
    pub const fn new(player_start: bool) -> Self {
        Self {
            x_turn: player_start,
            board: [[Tile::Empty; 15]; 15],
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_str = String::new();

        for row in &self.board {
            for tile in row {
                let tile_str = match tile {
                    Tile::Empty => " ",
                    Tile::X => "X",
                    Tile::O => "O",
                    Tile::None => panic!(),
                };
                board_str.push_str(&format!("| {tile_str} "));
            }
            board_str.push_str("|\n");
        }

        write!(f, "{board_str}")
    }
}
