use ffi::Rectangle;
use raylib::prelude::*;
use std::ops::Range;

use crate::snake::*;
use crate::utils::*;

pub struct Food {
    pub position: Vector2,
}

impl Food {
    pub fn new(ray: &RaylibHandle) -> Self {
        Food {
            position: Vector2::new(
                ray.get_random_value::<i32>(Range {
                    start: 1,
                    end: (MAP_SIZE - 2.0) as i32,
                }) as f32,
                ray.get_random_value::<i32>(Range {
                    start: 1,
                    end: (MAP_SIZE - 2.0) as i32,
                }) as f32,
            ),
        }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_rectangle_rec(
            Rectangle {
                x: self.position.x * TILE_SIZE,
                y: self.position.y * TILE_SIZE,
                width: TILE_SIZE,
                height: TILE_SIZE,
            },
            Color::MAGENTA,
        );
    }

    pub fn respawn(&mut self, ray: &RaylibHandle, snake: &Snake) {
        let mut valid_positions: Vec<Vector2> = Vec::new();

        // Get all the positions not occupied by snake
        for x in 1..(MAP_SIZE as i32 - 2) {
            for y in 1..(MAP_SIZE as i32 - 2) {
                let position = Vector2::new(x as f32, y as f32);
                if !snake.body.contains(&position) {
                    valid_positions.push(position);
                }
            }
        }

        // Get random position from the valid ones
        if !valid_positions.is_empty() {
            let index = ray.get_random_value::<i32>(Range {
                start: 0,
                end: (valid_positions.len() - 1) as i32,
            }) as usize;

            self.position = valid_positions[index];
        }
    }
}
