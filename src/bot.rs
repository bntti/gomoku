use crate::state::{GameState, Tile};
use macroquad::rand::ChooseRandom;

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

const ALPHA: f32 = -f32::INFINITY;
const BETA: f32 = f32::INFINITY;

fn move_gen(state: &GameState) -> Vec<(usize, usize)> {
    let max_dist = 2;
    let mut visited = [[false; 15]; 15];
    let mut active = vec![];
    for x in 0..15 {
        for y in 0..15 {
            if state.board[x][y] != Tile::Empty {
                active.push((0, x, y));
                visited[x][y] = true;
            }
        }
    }

    let mut result = vec![];
    while !active.is_empty() {
        let (dist, x, y) = active.pop().unwrap();
        for dir in DIRS {
            let nx = x as i32 + dir.0;
            let ny = y as i32 + dir.1;
            if nx < 0 || nx >= 15 || ny < 0 || ny >= 15 || visited[nx as usize][ny as usize] {
                continue;
            }
            visited[nx as usize][ny as usize] = true;
            if state.board[nx as usize][ny as usize] == Tile::Empty {
                result.push((nx as usize, ny as usize));
            }
            if dist < max_dist - 1 {
                active.push((dist + 1, nx as usize, ny as usize));
            }
        }
    }
    if result.is_empty() {
        for x in 0..15 {
            for y in 0..15 {
                result.push((x, y));
            }
        }
    }
    result.shuffle();

    result
}

fn eval_line(line: Vec<Tile>, maximize: bool) -> f32 {
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

        let player_turn = (tile == Tile::O && maximize) || (tile == Tile::X && !maximize);
        let player_sign = if tile == Tile::O { 1.0 } else { -1.0 };
        const INF: f32 = 10000000.0;

        if len == 1 {
            value += player_sign
                * match both_sides_empty {
                    false => 0.1,
                    true => 0.3,
                }
        } else if len == 2 {
            value += player_sign
                * match both_sides_empty {
                    false => 0.4,
                    true => 0.7,
                }
        } else if len == 3 {
            value += player_sign
                * match both_sides_empty {
                    false => 1.0,
                    true => {
                        if empty_before + empty_after >= 3 {
                            7.0
                        } else {
                            3.0
                        }
                    }
                }
        } else if len == 4 {
            value += player_sign
                * match both_sides_empty {
                    false => {
                        if player_turn {
                            INF / 10.0
                        } else {
                            10.0
                        }
                    }
                    true => {
                        if player_turn {
                            INF / 10.0
                        } else {
                            100.0
                        }
                    }
                }
        } else if len >= 5 {
            value += player_sign * INF;
        }
    }
    value
}

fn eval(state: &GameState, maximize: bool) -> f32 {
    let mut value = 0.0;

    // Vertical / horizontal
    for i in 0..15 {
        let mut line1 = vec![];
        let mut line2 = vec![];
        for j in 0..15 {
            line1.push(state.board[i][j]);
            line2.push(state.board[j][i]);
        }
        value += eval_line(line1, maximize);
        value += eval_line(line2, maximize);
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
        value += eval_line(line1, maximize);
        value += eval_line(line2, maximize);
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
        value += eval_line(line1, maximize);
        value += eval_line(line2, maximize);
    }

    value
}

fn alphabeta(state: &mut GameState, depth: i32, alpha: f32, beta: f32, maximize: bool) -> f32 {
    let evaluation = eval(state, maximize);
    if depth == 0 || f32::abs(evaluation) > 1000.0 {
        return evaluation;
    }

    let moves = move_gen(state);
    if maximize {
        let mut value = -f32::INFINITY;
        let mut alpha = alpha;
        for mv in moves {
            state.board[mv.0][mv.1] = Tile::O;
            value = f32::max(value, alphabeta(state, depth - 1, alpha, beta, !maximize));
            state.board[mv.0][mv.1] = Tile::Empty;

            if value > beta {
                break;
            }
            alpha = f32::max(alpha, value);
        }
        return value;
    } else {
        let mut value = f32::INFINITY;
        let mut beta = beta;
        for mv in moves {
            state.board[mv.0][mv.1] = Tile::X;
            value = f32::min(value, alphabeta(state, depth - 1, alpha, beta, !maximize));
            state.board[mv.0][mv.1] = Tile::Empty;

            if value < alpha {
                break;
            }
            beta = f32::min(beta, value);
        }
        return value;
    }
}

// TODO: remove maximize, it is included in the state.
pub fn bot_move(state: &mut GameState) -> (usize, usize) {
    // ------------------------------------------------------------------------
    // let mut test = GameState::new(false);
    // let get_move = false;
    // let bot_move = false;
    //
    // // test.board[1][1] = Tile::O;
    // test.board[2][2] = Tile::O;
    // test.board[3][2] = Tile::O;
    // test.board[5][2] = Tile::O;
    // // test.board[8][8] = Tile::O;
    // // test.board[8][8] = Tile::X;
    //
    // // test.board[4][5] = Tile::O;
    // // test.board[4][6] = Tile::O;
    // // test.board[4][7] = Tile::O;
    //
    // println!("Testing state:\n{}", test);
    //
    // if get_move {
    //     let moves = move_gen(&test);
    //     let mut best_move = (0, 0);
    //     let mut value = -f32::INFINITY;
    //     for mv in moves {
    //         test.board[mv.0][mv.1] = Tile::O;
    //
    //         let new_val = alphabeta(&mut test, 1, ALPHA, BETA, false);
    //         // println!("state:\n{}eval: {}", test, new_val);
    //         if new_val > value {
    //             value = new_val;
    //             best_move = mv;
    //         }
    //
    //         test.board[mv.0][mv.1] = Tile::Empty;
    //     }
    //     println!("val: {}, move: {:?}", value, best_move);
    //
    //     // println!("{}", test);
    //     test.board[best_move.0][best_move.1] = Tile::O;
    //     println!("{}", test);
    // } else {
    //     println!("eval {}", alphabeta(&mut test, 2, ALPHA, BETA, true));
    //     // println!("eval {}", eval(&test, bot_move));
    // }
    //
    // panic!();
    // ------------------------------------------------------------------------

    let moves = move_gen(state);
    let mut best_move = (0, 0);
    if !state.player_turn {
        let mut value = -f32::INFINITY;
        for mv in moves {
            state.board[mv.0][mv.1] = Tile::O;
            let new_val = alphabeta(state, 2, ALPHA, BETA, false);
            state.board[mv.0][mv.1] = Tile::Empty;

            if new_val > value {
                value = new_val;
                best_move = mv;
            }
        }
        println!("Eval: {:.2}", value);
    } else {
        let mut value = f32::INFINITY;
        for mv in moves {
            state.board[mv.0][mv.1] = Tile::X;
            let new_val = alphabeta(state, 2, ALPHA, BETA, true);
            state.board[mv.0][mv.1] = Tile::Empty;

            if new_val < value {
                value = new_val;
                best_move = mv;
            }
        }

        println!("Eval: {:.2}", value);
    }

    best_move
}
