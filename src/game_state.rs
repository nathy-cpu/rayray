use ffi::Rectangle;
use raylib::prelude::*;

use crate::utils::*;

pub struct GameState {
    pub current_score: u32,
    pub highest_score: u32,
    pub status: GameStatus,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            current_score: 0,
            highest_score: 0,
            status: GameStatus::Paused,
        }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        let score = &format!("SCORE: {}", self.current_score);
        let high_score = &format!("HIGHEST SCORE: {}", self.current_score);
        let score_length = draw_handle.measure_text(score, FONT_SIZE as i32);
        let high_score_length = draw_handle.measure_text(high_score, FONT_SIZE as i32);

        draw_handle.draw_text(
            score,
            (SCREEN_WIDTH / 2.0) as i32 - (score_length / 2) as i32,
            (TILE_SIZE * 0.25) as i32,
            FONT_SIZE as i32,
            Color::WHITE,
        );

        match self.status {
            GameStatus::Playing => {}
            GameStatus::Paused => {
                let paused = "GAME PAUSED";
                let paused_length = draw_handle.measure_text(paused, FONT_SIZE as i32);

                draw_handle.draw_rectangle_rec(
                    Rectangle {
                        x: (SCREEN_WIDTH / 2.0) - ((paused_length / 2) as f32 + FONT_SIZE),
                        y: (SCREEN_HEIGHT / 2.0),
                        width: (FONT_SIZE * 2.0) + paused_length as f32,
                        height: TILE_SIZE,
                    },
                    Color::YELLOWGREEN,
                );

                draw_handle.draw_text(
                    paused,
                    (SCREEN_WIDTH / 2.0) as i32 - (paused_length / 2),
                    (SCREEN_HEIGHT / 2.0) as i32,
                    FONT_SIZE as i32,
                    Color::WHITE,
                );
            }
            GameStatus::GameOver => {
                let game_over = "GAME OVER!";
                let game_over_length = draw_handle.measure_text(game_over, FONT_SIZE as i32);

                draw_handle.draw_rectangle_rec(
                    Rectangle {
                        x: (SCREEN_WIDTH / 2.0) - ((high_score_length / 2) as f32 + FONT_SIZE),
                        y: (((MAP_SIZE / 2.0) - 1.0) * TILE_SIZE),
                        width: (FONT_SIZE * 2.0) + high_score_length as f32,
                        height: TILE_SIZE * 2.0,
                    },
                    Color::GREEN,
                );

                draw_handle.draw_text(
                    game_over,
                    (SCREEN_WIDTH / 2.0) as i32 - (game_over_length / 2),
                    (((MAP_SIZE / 2.0) - 1.0) * TILE_SIZE) as i32,
                    FONT_SIZE as i32,
                    Color::WHITE,
                );

                draw_handle.draw_text(
                    high_score,
                    (SCREEN_WIDTH / 2.0) as i32 - (high_score_length / 2),
                    (MAP_SIZE / 2.0 * TILE_SIZE) as i32,
                    FONT_SIZE as i32,
                    Color::WHITE,
                );
            }
        }
    }
}
