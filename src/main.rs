use raylib::color::Color;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

mod food;
mod game_state;
mod snake;
mod utils;

use food::*;
use game_state::*;
use snake::*;
use utils::*;

fn main() {
    let (mut ray, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("RayRay")
        .vsync()
        .build();

    ray.set_target_fps(10);

    let mut snake = Snake::new();
    let mut food = Food::new(&ray);
    let mut state = GameState::new();

    while !ray.window_should_close() {
        /**** Update ****/

        // Check GameStatus
        if ray.is_key_pressed(KEY_SPACE) {
            state.status = match state.status {
                GameStatus::Paused => GameStatus::Playing,
                GameStatus::Playing => GameStatus::Paused,
                GameStatus::GameOver => {
                    state.current_score = 0;
                    snake = Snake::new();
                    food = Food::new(&ray);

                    GameStatus::Playing
                }
            }
        }

        // Only update if GameStatus::Playing
        if state.status == GameStatus::Playing {
            if ray.is_key_pressed(KEY_UP) {
                snake.next_direction = Direction::Up;
            }
            if ray.is_key_pressed(KEY_DOWN) {
                snake.next_direction = Direction::Down;
            }
            if ray.is_key_pressed(KEY_LEFT) {
                snake.next_direction = Direction::Left;
            }
            if ray.is_key_pressed(KEY_RIGHT) {
                snake.next_direction = Direction::Right;
            }

            snake.update();

            // Check snake collision with itself
            for tile in snake.body.iter().skip(1) {
                if snake.body[0] == *tile {
                    state.status = GameStatus::GameOver;
                    state.highest_score = std::cmp::max(state.highest_score, state.current_score);
                    snake = Snake::new();
                    food = Food::new(&ray);
                    break;
                }
            }

            // Check snake collision with food
            if snake.body[0] == food.position {
                food.respawn(&ray, &snake);
                while snake.body.iter().filter(|x| **x == food.position).count() != 0 {
                    food.respawn(&ray, &snake);
                }

                let temp = *snake.body.back().unwrap();
                snake.body.push_back(temp);
                state.current_score += 1;
            }
        }

        /**** Draw ****/

        let mut draw_handle = ray.begin_drawing(&thread);

        draw_handle.clear_background(Color::BLACK);

        snake.draw(&mut draw_handle);
        food.draw(&mut draw_handle);
        draw_border(&mut draw_handle, Color::BROWN);
        state.draw(&mut draw_handle);
    }
}
