use crate::eval::eval;
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

const MAX_DIST: i32 = 2;
const DEPTH: i32 = 2;
const ALPHA: f32 = -f32::INFINITY;
const BETA: f32 = f32::INFINITY;

fn move_gen(state: &GameState) -> Vec<(usize, usize)> {
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
            if !(0..15).contains(&nx) || !(0..15).contains(&ny) || visited[nx as usize][ny as usize]
            {
                continue;
            }
            visited[nx as usize][ny as usize] = true;
            if state.board[nx as usize][ny as usize] == Tile::Empty {
                result.push((nx as usize, ny as usize));
            }
            if dist < MAX_DIST - 1 {
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

fn alphabeta(state: &mut GameState, depth: i32, alpha: f32, beta: f32) -> f32 {
    let evaluation = eval(state);
    if depth == 0 || f32::abs(evaluation) > 1000.0 {
        return evaluation;
    }

    let moves = move_gen(state);
    let maximize = !state.x_turn;
    if maximize {
        let mut value = -f32::INFINITY;
        let mut alpha = alpha;
        for mv in moves {
            state.board[mv.0][mv.1] = Tile::O;
            state.x_turn = !state.x_turn;
            value = f32::max(value, alphabeta(state, depth - 1, alpha, beta));
            state.x_turn = !state.x_turn;
            state.board[mv.0][mv.1] = Tile::Empty;

            if value > beta {
                break;
            }
            alpha = f32::max(alpha, value);
        }
        value
    } else {
        let mut value = f32::INFINITY;
        let mut beta = beta;
        for mv in moves {
            state.board[mv.0][mv.1] = Tile::X;
            state.x_turn = !state.x_turn;
            value = f32::min(value, alphabeta(state, depth - 1, alpha, beta));
            state.x_turn = !state.x_turn;
            state.board[mv.0][mv.1] = Tile::Empty;

            if value < alpha {
                break;
            }
            beta = f32::min(beta, value);
        }
        value
    }
}

pub fn bot_move(state: &mut GameState) -> (usize, usize) {
    let moves = move_gen(state);
    let mut best_move = (0, 0);
    if state.x_turn {
        let mut value = f32::INFINITY;
        for mv in moves {
            state.board[mv.0][mv.1] = Tile::X;
            state.x_turn = !state.x_turn;
            let new_val = alphabeta(state, DEPTH, ALPHA, BETA);
            state.x_turn = !state.x_turn;
            state.board[mv.0][mv.1] = Tile::Empty;

            if new_val < value {
                value = new_val;
                best_move = mv;
            }
        }
        println!("Eval: {value:.2}");
    } else {
        let mut value = -f32::INFINITY;
        for mv in moves {
            state.board[mv.0][mv.1] = Tile::O;
            state.x_turn = !state.x_turn;
            let new_val = alphabeta(state, DEPTH, ALPHA, BETA);
            state.x_turn = !state.x_turn;
            state.board[mv.0][mv.1] = Tile::Empty;

            if new_val > value {
                value = new_val;
                best_move = mv;
            }
        }
        println!("Eval: {value:.2}");
    }

    best_move
}
