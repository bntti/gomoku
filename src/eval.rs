use crate::state::{GameState, Tile};

const INF: f32 = 100_000_000.0;
fn eval_line(line: Vec<Tile>, x_turn: bool) -> f32 {
    if line.len() < 5 {
        return 0.0;
    }
    let mut tmp_line = line;
    tmp_line.push(Tile::None);

    let mut line = vec![];
    let mut len = 0;
    let mut prev_tile = Tile::None;
    for tile in tmp_line {
        if tile == prev_tile {
            len += 1;
        } else {
            line.push((prev_tile, len));
            len = 1;
            prev_tile = tile;
        }
    }
    line.push((Tile::None, 1));

    let mut value = 0.0;
    for i in 1..line.len() - 1 {
        let (tile, len) = line[i];
        if tile == Tile::Empty {
            continue;
        }

        let empty_before = match line[i - 1].0 {
            Tile::Empty => line[i - 1].1,
            _ => 0,
        };
        let empty_after = match line[i + 1].0 {
            Tile::Empty => line[i + 1].1,
            _ => 0,
        };

        if line[i].1 + empty_before + empty_after < 5 {
            continue;
        }

        let both_sides_empty = (empty_before > 0) && (empty_after > 0);

        let player_turn = (tile == Tile::O && !x_turn) || (tile == Tile::X && x_turn);
        let player_sign = if tile == Tile::O { 1.0 } else { -1.0 };

        if len == 1 {
            value += player_sign * if !both_sides_empty { 0.1 } else { 0.3 }
        } else if len == 2 {
            value += player_sign * if !both_sides_empty { 0.4 } else { 0.7 }
        } else if len == 3 {
            value += player_sign
                * if !both_sides_empty {
                    1.0
                } else if empty_before + empty_after >= 3 {
                    7.0
                } else {
                    3.0
                }
        } else if len == 4 {
            value += player_sign
                * if player_turn {
                    INF / 10.0
                } else if !both_sides_empty {
                    10.0
                } else {
                    100.0
                }
        } else if len >= 5 {
            value += player_sign * INF;
        }
    }
    value
}

pub fn eval(state: &GameState) -> f32 {
    let mut value = 0.0;

    // Vertical / horizontal
    for i in 0..15 {
        let mut line1 = vec![];
        let mut line2 = vec![];
        for j in 0..15 {
            line1.push(state.board[i][j]);
            line2.push(state.board[j][i]);
        }
        value += eval_line(line1, state.x_turn);
        value += eval_line(line2, state.x_turn);
    }

    // Diagonal from top
    for i in 0..15 {
        let mut line1 = vec![];
        let mut line2 = vec![];
        for j in 0..15 {
            if i as i32 - j as i32 >= 0 {
                line1.push(state.board[i - j][j]);
            };
            if i + j < 15 {
                line2.push(state.board[i + j][j]);
            }
        }
        value += eval_line(line1, state.x_turn);
        value += eval_line(line2, state.x_turn);
    }

    // Diagonal from bottom
    for i in 1..14 {
        let mut line1 = vec![];
        let mut line2 = vec![];
        for j in 0..15 {
            if i as i32 - j as i32 >= 0 {
                line1.push(state.board[i - j][14 - j]);
            };
            if i + j < 15 {
                line2.push(state.board[i + j][14 - j]);
            }
        }
        value += eval_line(line1, state.x_turn);
        value += eval_line(line2, state.x_turn);
    }

    value
}
