use raylib::prelude::*;
use std::collections::VecDeque;

use crate::utils::*;

pub struct Snake {
    pub body: VecDeque<Vector2>,
    pub direction: Direction,
    pub next_direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
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

    pub fn update(&mut self) {
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

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
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
