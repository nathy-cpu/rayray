use raylib::prelude::*;

pub const TILE_SIZE: f32 = 30.0;
pub const MAP_SIZE: f32 = 30.0;
pub const SCREEN_WIDTH: f32 = TILE_SIZE * MAP_SIZE;
pub const SCREEN_HEIGHT: f32 = TILE_SIZE * MAP_SIZE;
pub const FONT_SIZE: f32 = TILE_SIZE * 0.7;

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(PartialEq, Clone, Copy)]
pub enum GameStatus {
    Playing,
    Paused,
    GameOver,
}
pub fn draw_border(draw_handle: &mut RaylibDrawHandle, color: Color) {
    // Top border
    draw_handle.draw_rectangle_rec(
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: SCREEN_WIDTH,
            height: TILE_SIZE,
        },
        color,
    );

    // Left border
    draw_handle.draw_rectangle_rec(
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: TILE_SIZE,
            height: SCREEN_HEIGHT,
        },
        color,
    );

    // Right border
    draw_handle.draw_rectangle_rec(
        Rectangle {
            x: SCREEN_WIDTH - TILE_SIZE,
            y: 0.0,
            width: TILE_SIZE,
            height: SCREEN_HEIGHT,
        },
        color,
    );

    // Bottom border
    draw_handle.draw_rectangle_rec(
        Rectangle {
            x: 0.0,
            y: SCREEN_HEIGHT - TILE_SIZE,
            width: SCREEN_WIDTH,
            height: TILE_SIZE,
        },
        color,
    );
}
