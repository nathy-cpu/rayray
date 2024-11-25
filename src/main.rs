use ffi::Rectangle;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use std::collections::VecDeque;
use std::ops::Range;

const TILE_SIZE: f32 = 20.0;
const MAP_SIZE: f32 = 30.0;
const SCREEN_WIDTH: f32 = TILE_SIZE * MAP_SIZE;
const SCREEN_HEIGHT: f32 = TILE_SIZE * MAP_SIZE;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

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

struct Snake {
    body: VecDeque<Vector2>,
    direction: Direction,
    next_direction: Direction,
}

impl Snake {
    fn new() -> Self {
        let mut snake = Snake {
            body: VecDeque::new(),
            direction: Direction::RIGHT,
            next_direction: Direction::LEFT,
        };

        snake.body.push_back(Vector2::new(4.0, MAP_SIZE * 0.5));
        snake.body.push_back(Vector2::new(3.0, MAP_SIZE * 0.5));
        snake.body.push_back(Vector2::new(2.0, MAP_SIZE * 0.5));

        return snake;
    }

    fn update(&mut self) {
        let _ = self.body.pop_back();
        let mut temp = self.body[0].clone();

        if self.direction != self.next_direction {
            match self.direction {
                Direction::UP => {
                    if self.next_direction == Direction::DOWN {
                        self.next_direction = Direction::UP;
                    }
                }

                Direction::DOWN => {
                    if self.next_direction == Direction::UP {
                        self.next_direction = Direction::DOWN;
                    }
                }

                Direction::LEFT => {
                    if self.next_direction == Direction::RIGHT {
                        self.next_direction = Direction::LEFT;
                    }
                }

                Direction::RIGHT => {
                    if self.next_direction == Direction::LEFT {
                        self.next_direction = Direction::RIGHT;
                    }
                }
            }

            self.direction = self.next_direction;
        }

        match self.direction {
            Direction::UP => {
                temp.y -= 1.0;
                if temp.y <= 0.0 {
                    temp.y = MAP_SIZE - 2.0;
                }
                self.body.push_front(temp);
            }

            Direction::DOWN => {
                temp.y += 1.0;
                if temp.y >= MAP_SIZE - 1.0 {
                    temp.y = 1.0;
                }
                self.body.push_front(temp);
            }

            Direction::LEFT => {
                temp.x -= 1.0;
                if temp.x <= 0.0 {
                    temp.x = MAP_SIZE - 2.0;
                }
                self.body.push_front(temp);
            }

            Direction::RIGHT => {
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
                    start: 1,
                    end: MAP_SIZE as i32 - 1,
                }) as f32,
                ray.get_random_value::<i32>(Range {
                    start: 1,
                    end: MAP_SIZE as i32 - 1,
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
                start: 1,
                end: MAP_SIZE as i32 - 1,
            }) as f32,
            ray.get_random_value::<i32>(Range {
                start: 1,
                end: MAP_SIZE as i32 - 1,
            }) as f32,
        );
    }
}

fn main() {
    let (mut ray, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("RayRay")
        .vsync()
        .build();

    ray.set_target_fps(12);

    let mut snake = Snake::new();
    let mut food = Food::new(&ray);
    let mut _state = GameState {
        current_score: 0,
        highest_score: 0,
        status: GameStatus::Paused,
    };

    while !ray.window_should_close() {
        // Update
        if ray.is_key_pressed(KEY_UP) {
            snake.next_direction = Direction::UP;
        }
        if ray.is_key_pressed(KEY_DOWN) {
            snake.next_direction = Direction::DOWN;
        }
        if ray.is_key_pressed(KEY_LEFT) {
            snake.next_direction = Direction::LEFT;
        }
        if ray.is_key_pressed(KEY_RIGHT) {
            snake.next_direction = Direction::RIGHT;
        }

        snake.update();

        for tile in snake.body.iter().skip(1) {
            if snake.body[0] == *tile {
                return;
            }
        }

        if snake.body[0] == food.position {
            food.respawn(&ray);
            while snake.body.iter().filter(|x| **x == food.position).count() != 0 {
                food.respawn(&ray);
            }

            let temp = snake.body.back().unwrap().clone();
            snake.body.push_back(temp);
        }

        // Draw
        let mut draw_handle = ray.begin_drawing(&thread);

        draw_handle.clear_background(Color::BLACK);

        // Top border
        draw_handle.draw_rectangle_rec(
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: SCREEN_WIDTH,
                height: TILE_SIZE,
            },
            Color::RED,
        );

        // Left border
        draw_handle.draw_rectangle_rec(
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: TILE_SIZE,
                height: SCREEN_HEIGHT,
            },
            Color::RED,
        );

        // Right border
        draw_handle.draw_rectangle_rec(
            Rectangle {
                x: SCREEN_WIDTH - TILE_SIZE,
                y: 0.0,
                width: TILE_SIZE,
                height: SCREEN_HEIGHT,
            },
            Color::RED,
        );

        // Bottom border
        draw_handle.draw_rectangle_rec(
            Rectangle {
                x: 0.0,
                y: SCREEN_HEIGHT - TILE_SIZE,
                width: SCREEN_WIDTH,
                height: TILE_SIZE,
            },
            Color::RED,
        );

        snake.draw(&mut draw_handle);
        food.draw(&mut draw_handle);
    }
}
