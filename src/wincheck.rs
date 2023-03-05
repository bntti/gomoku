use crate::state::{GameState, Tile};

fn eval_line(line: Vec<Tile>) -> bool {
    if line.len() < 5 {
        return false;
    }
    let mut tmp_line = line;
    tmp_line.push(Tile::None);

    let mut len = 0;
    let mut prev_tile = Tile::None;
    for tile in tmp_line {
        if len >= 5 && (prev_tile == Tile::X || prev_tile == Tile::O) {
            return true;
        }
        if tile == prev_tile {
            len += 1;
        } else {
            len = 1;
            prev_tile = tile;
        }
    }
    false
}

pub fn check_win(state: &GameState) -> bool {
    // Vertical / horizontal
    for i in 0..15 {
        let mut line1 = vec![];
        let mut line2 = vec![];
        for j in 0..15 {
            line1.push(state.board[i][j]);
            line2.push(state.board[j][i]);
        }
        if eval_line(line1) {
            return true;
        }
        if eval_line(line2) {
            return true;
        }
    }

    // Diagonal from top
    for i in 0..15 {
        let mut line1 = vec![];
        let mut line2 = vec![];
        for j in 0..15 {
            if i as i32 - j as i32 >= 0 {
                line1.push(state.board[i - j][j])
            };
            if i + j < 15 {
                line2.push(state.board[i + j][j]);
            }
        }

        if eval_line(line1) {
            return true;
        }
        if eval_line(line2) {
            return true;
        }
    }

    // Diagonal from bottom
    for i in 1..14 {
        let mut line1 = vec![];
        let mut line2 = vec![];
        for j in 0..15 {
            if i as i32 - j as i32 >= 0 {
                line1.push(state.board[i - j][14 - j])
            };
            if i + j < 15 {
                line2.push(state.board[i + j][14 - j]);
            }
        }

        if eval_line(line1) {
            return true;
        }
        if eval_line(line2) {
            return true;
        }
    }

    false
}
