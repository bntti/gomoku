use bot::bot_move;
use macroquad::prelude::*;
use state::{GameState, Tile};

mod bot;
mod state;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut state = GameState::new(true);

    loop {
        let tile_size = f32::min(screen_width() / 15.0, screen_height() / 15.0);

        if let Some(_) = get_last_key_pressed() {
            break;
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
                    draw_circle(center_x, center_y, tile_size / 2.0 - padding, RED);
                }
            }
        }

        next_frame().await
    }
}
