use bot::bot_move;
use macroquad::prelude::*;
use state::{GameState, Tile};
use wincheck::check_win;

mod bot;
mod state;
mod wincheck;

const BOT_VS_BOT: bool = false;
const SHOW_RECOMMENDATION: bool = false;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut state = GameState::new(true);
    rand::srand(macroquad::miniquad::date::now() as _);

    let mut prev_o_move = (-1, -1);
    let mut prev_x_move = (-1, -1);
    let mut recommendation = (7, 7);
    let mut recommendation_frame = 0;
    loop {
        let tile_size = f32::min(screen_width() / 15.0, screen_height() / 15.0);

        if let Some(keypress) = get_last_key_pressed() {
            if keypress == macroquad::input::KeyCode::Q {
                break;
            }
        }

        recommendation_frame += 1;
        let game_over = check_win(&state);
        if state.player_turn && !BOT_VS_BOT && !game_over {
            let (mouse_x, mouse_y) = mouse_position();
            if is_mouse_button_down(MouseButton::Left) {
                let x = f32::floor(mouse_x / tile_size) as usize;
                let y = f32::floor(mouse_y / tile_size) as usize;
                if x <= 14 && y <= 14 && state.board[x][y] == Tile::Empty {
                    state.board[x][y] = Tile::X;
                    state.player_turn = false;
                }
                prev_x_move = (x as i32, y as i32);
            }
        } else if state.player_turn && BOT_VS_BOT && !game_over {
            let (x, y) = bot_move(&mut state);
            prev_x_move = (x as i32, y as i32);

            if x <= 14 && y <= 14 && state.board[x][y] == Tile::Empty {
                state.board[x][y] = Tile::X;
                state.player_turn = false;
            } else {
                panic!("Bot made illegal move");
            }
        } else if !state.player_turn && !game_over {
            let (x, y) = bot_move(&mut state);
            prev_o_move = (x as i32, y as i32);

            if x <= 14 && y <= 14 && state.board[x][y] == Tile::Empty {
                state.board[x][y] = Tile::O;
                state.player_turn = true;
            } else {
                panic!("Bot made illegal move");
            }

            if SHOW_RECOMMENDATION && !BOT_VS_BOT {
                recommendation = bot_move(&mut state);
            }
            recommendation_frame = 0;
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

        // Display previous move
        if state.player_turn && prev_o_move.1 >= 0 {
            let center_x = prev_o_move.0 as f32 * tile_size + tile_size / 2.0;
            let center_y = prev_o_move.1 as f32 * tile_size + tile_size / 2.0;
            draw_circle(center_x, center_y, tile_size / 2.0 - padding, RED);
        } else if !state.player_turn && prev_x_move.1 >= 0 {
            let half_size = (tile_size) / 2.0 - padding;
            let center_x = prev_x_move.0 as f32 * tile_size + tile_size / 2.0;
            let center_y = prev_x_move.1 as f32 * tile_size + tile_size / 2.0;
            draw_line(
                center_x - half_size,
                center_y - half_size,
                center_x + half_size,
                center_y + half_size,
                3.0,
                BLUE,
            );
            draw_line(
                center_x - half_size,
                center_y + half_size,
                center_x + half_size,
                center_y - half_size,
                3.0,
                BLUE,
            );
        }

        // Show recommendation after some time
        if SHOW_RECOMMENDATION && !BOT_VS_BOT {
            recommendation_frame = i32::min(recommendation_frame, 5000);
            let color = Color::new(0.0, 1.0, 0.0, recommendation_frame as f32 / 5000.0);
            let half_size = (tile_size) / 2.0 - padding;
            let center_x = recommendation.0 as f32 * tile_size + tile_size / 2.0;
            let center_y = recommendation.1 as f32 * tile_size + tile_size / 2.0;
            draw_line(
                center_x - half_size,
                center_y - half_size,
                center_x + half_size,
                center_y + half_size,
                1.0,
                color,
            );
            draw_line(
                center_x - half_size,
                center_y + half_size,
                center_x + half_size,
                center_y - half_size,
                1.0,
                color,
            );
        }

        // Draw O:s and X:s
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
