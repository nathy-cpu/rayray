use ffi::Rectangle;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use std::collections::VecDeque;
use std::ops::Range;

const TILE_SIZE: f32 = 30.0;
const MAP_SIZE: f32 = 25.0;
const SCREEN_WIDTH: f32 = TILE_SIZE * MAP_SIZE;
const SCREEN_HEIGHT: f32 = TILE_SIZE * MAP_SIZE;
const FONT_SIZE: f32 = TILE_SIZE * 0.75;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(PartialEq, Clone, Copy)]
enum GameStatus {
    Playing,
    Paused,
    GameOver,
}

struct GameState {
    current_score: u32,
    highest_score: u32,
    status: GameStatus,
}

impl GameState {
    fn new() -> Self {
        GameState {
            current_score: 0,
            highest_score: 0,
            status: GameStatus::Paused,
        }
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
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
                        x: (SCREEN_WIDTH / 2.0) - (paused_length / 2) as f32 + FONT_SIZE,
                        y: (SCREEN_HEIGHT / 2.0),
                        width: (FONT_SIZE * 2.0) + paused_length as f32,
                        height: TILE_SIZE,
                    },
                    Color::RED,
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
                        x: (SCREEN_WIDTH / 2.0) - (high_score_length / 2) as f32 + FONT_SIZE,
                        y: (((MAP_SIZE / 2.0) - 1.0) * TILE_SIZE),
                        width: (FONT_SIZE * 2.0) + high_score_length as f32,
                        height: TILE_SIZE * 2.0,
                    },
                    Color::RED,
                );

                draw_handle.draw_text(
                    game_over,
                    (SCREEN_WIDTH / 2.0) as i32 - (game_over_length / 2),
                    (((MAP_SIZE / 2.0) - 1.0) * TILE_SIZE) as i32,
                    FONT_SIZE as i32,
                    Color::GREEN,
                );

                draw_handle.draw_text(
                    high_score,
                    (SCREEN_WIDTH / 2.0) as i32 - (high_score_length / 2),
                    (MAP_SIZE / 2.0 * TILE_SIZE) as i32,
                    FONT_SIZE as i32,
                    Color::GREEN,
                );
            }
        }
    }
}

struct Snake {
    body: VecDeque<Vector2>,
    direction: Direction,
    next_direction: Direction,
}

impl Snake {
    fn new() -> Self {
        let mut snake = Snake {
            body: VecDeque::new(),
            direction: Direction::Right,
            next_direction: Direction::Left,
        };

        snake.body.push_back(Vector2::new(4.0, MAP_SIZE * 0.5));
        snake.body.push_back(Vector2::new(3.0, MAP_SIZE * 0.5));
        snake.body.push_back(Vector2::new(2.0, MAP_SIZE * 0.5));

        snake
    }

    fn update(&mut self) {
        let _ = self.body.pop_back();
        let mut temp = self.body[0];

        if self.direction != self.next_direction {
            match self.direction {
                Direction::Up => {
                    if self.next_direction == Direction::Down {
                        self.next_direction = Direction::Up;
                    }
                }

                Direction::Down => {
                    if self.next_direction == Direction::Up {
                        self.next_direction = Direction::Down;
                    }
                }

                Direction::Left => {
                    if self.next_direction == Direction::Right {
                        self.next_direction = Direction::Left;
                    }
                }

                Direction::Right => {
                    if self.next_direction == Direction::Left {
                        self.next_direction = Direction::Right;
                    }
                }
            }

            self.direction = self.next_direction;
        }

        match self.direction {
            Direction::Up => {
                temp.y -= 1.0;
                if temp.y <= 0.0 {
                    temp.y = MAP_SIZE - 2.0;
                }
                self.body.push_front(temp);
            }

            Direction::Down => {
                temp.y += 1.0;
                if temp.y >= MAP_SIZE - 1.0 {
                    temp.y = 1.0;
                }
                self.body.push_front(temp);
            }

            Direction::Left => {
                temp.x -= 1.0;
                if temp.x <= 0.0 {
                    temp.x = MAP_SIZE - 2.0;
                }
                self.body.push_front(temp);
            }

            Direction::Right => {
                temp.x += 1.0;
                if temp.x >= MAP_SIZE - 1.0 {
                    temp.x = 1.0;
                }
                self.body.push_front(temp);
            }
        }
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        let mut color = Color::GREEN;

        for tile in self.body.iter() {
            draw_handle.draw_rectangle_rec(
                Rectangle {
                    x: tile.x * TILE_SIZE,
                    y: tile.y * TILE_SIZE,
                    width: TILE_SIZE,
                    height: TILE_SIZE,
                },
                color,
            );

            if color == Color::GREEN {
                color = Color::RAYWHITE;
            }
        }
    }
}

struct Food {
    position: Vector2,
}

impl Food {
    fn new(ray: &RaylibHandle) -> Self {
        Food {
            position: Vector2::new(
                ray.get_random_value::<i32>(Range {
                    start: 1 as i32,
                    end: (MAP_SIZE - 2.0) as i32,
                }) as f32,
                ray.get_random_value::<i32>(Range {
                    start: 1 as i32,
                    end: (MAP_SIZE - 2.0) as i32,
                }) as f32,
            ),
        }
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
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

    fn respawn(&mut self, ray: &RaylibHandle) {
        self.position = Vector2::new(
            ray.get_random_value::<i32>(Range {
                start: 1 as i32,
                end: (MAP_SIZE - 2.0) as i32,
            }) as f32,
            ray.get_random_value::<i32>(Range {
                start: 1 as i32,
                end: (MAP_SIZE - 2.0) as i32,
            }) as f32,
        );
    }
}

fn draw_border(draw_handle: &mut RaylibDrawHandle, color: Color) {
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
                food.respawn(&ray);
                while snake.body.iter().filter(|x| **x == food.position).count() != 0 {
                    food.respawn(&ray);
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
        draw_border(&mut draw_handle, Color::RED);
        state.draw(&mut draw_handle);
    }
}
