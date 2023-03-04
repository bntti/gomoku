use bot::bot_move;
use macroquad::prelude::*;
use state::{GameState, Tile};

mod bot;
mod state;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut state = GameState::new(true);

    let mut prev_bot_move = (-1, -1);
    loop {
        let tile_size = f32::min(screen_width() / 15.0, screen_height() / 15.0);

        if let Some(keypress) = get_last_key_pressed() {
            if keypress == macroquad::input::KeyCode::Q {
                break;
            }
        }

        if state.player_turn {
            let (mouse_x, mouse_y) = mouse_position();
            if is_mouse_button_down(MouseButton::Left) {
                let x = f32::floor(mouse_x / tile_size) as usize;
                let y = f32::floor(mouse_y / tile_size) as usize;
                if x <= 14 && y <= 14 && state.board[x][y] == Tile::Empty {
                    state.board[x][y] = Tile::X;
                    state.player_turn = false;
                }
            }
        } else {
            let (x, y) = bot_move(&mut state);
            prev_bot_move = (x as i32, y as i32);

            if x <= 14 && y <= 14 && state.board[x][y] == Tile::Empty {
                state.board[x][y] = Tile::O;
                state.player_turn = true;
            } else {
                panic!("Bot made illegal move");
            }
        }

        // Draw
        clear_background(BLACK);
        let padding = tile_size / 5.0;

        for x in 0..=15 {
            draw_line(
                x as f32 * tile_size,
                0.0,
                x as f32 * tile_size,
                tile_size * 15.0,
                1.0,
                WHITE,
            );
        }
        for y in 0..=15 {
            draw_line(
                0.0,
                y as f32 * tile_size,
                tile_size * 15.0,
                y as f32 * tile_size,
                1.0,
                WHITE,
            );
        }
        if state.player_turn && prev_bot_move.1 >= 0 {
            let center_x = prev_bot_move.0 as f32 * tile_size + tile_size / 2.0;
            let center_y = prev_bot_move.1 as f32 * tile_size + tile_size / 2.0;
            draw_circle(center_x, center_y, tile_size / 2.0 - padding, RED);
        }

        for x in 0..15 {
            for y in 0..15 {
                if state.board[x][y] == Tile::X {
                    let half_size = (tile_size) / 2.0 - padding;
                    let center_x = x as f32 * tile_size + tile_size / 2.0;
                    let center_y = y as f32 * tile_size + tile_size / 2.0;
                    draw_line(
                        center_x - half_size,
                        center_y - half_size,
                        center_x + half_size,
                        center_y + half_size,
                        1.0,
                        BLUE,
                    );
                    draw_line(
                        center_x - half_size,
                        center_y + half_size,
                        center_x + half_size,
                        center_y - half_size,
                        1.0,
                        BLUE,
                    );
                }
                if state.board[x][y] == Tile::O {
                    let center_x = x as f32 * tile_size + tile_size / 2.0;
                    let center_y = y as f32 * tile_size + tile_size / 2.0;
                    draw_circle_lines(center_x, center_y, tile_size / 2.0 - padding, 1.0, RED);
                }
            }
        }

        next_frame().await
    }
}
