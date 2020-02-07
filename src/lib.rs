mod utils;

extern crate rand;

use rand::Rng;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    None = 0,
    Left = 1,
    Right = 2,
    Up = 3,
    Down = 4,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellType {
    Empty = 0,
    Wall = 1,
    Food = 2,
    Snake = 3,
}

#[wasm_bindgen]
pub struct Snake {
    direction: Direction,
    head: (u8, u8),
    body: Vec<(u8, u8)>,
}

#[wasm_bindgen]
pub struct Field {
    width: u8,
    height: u8,
    snake: Snake,
    food: (u8, u8),
}

#[wasm_bindgen]
impl Field {
    pub fn new(width: u8, height: u8) -> Field {
        let head_pos = (
            width / 2,
            height / 2
        );

        Field {
            width,
            height,
            snake: Snake {
                direction: Direction::None,
                body: Vec::new(),
                head: head_pos,
            },
            food: Field::generate_random_cell(width, height),
        }
    }

    pub fn change_dir(&mut self, new_dir: Direction) {
        match (self.snake.direction, new_dir) {
            (Direction::Down, Direction::Up) => return,
            (Direction::Up, Direction::Down) => return,
            (Direction::Left, Direction::Right) => return,
            (Direction::Right, Direction::Left) => return,
            _ => {
                self.snake.direction = new_dir
            }
        }
    }

    pub fn tick(&mut self) {
        let new_head_pos = match self.snake.direction {
            Direction::Down => (self.snake.head.0, self.snake.head.1 + 1),
            Direction::Left => (self.snake.head.0 - 1, self.snake.head.1),
            Direction::Up => (self.snake.head.0, self.snake.head.1 - 1),
            Direction::Right => (self.snake.head.0 + 1, self.snake.head.1),
            _ => self.snake.head,
        };

        self.snake.body.insert(0, self.snake.head);

        if self.food == self.snake.head {
            self.food = Field::generate_random_cell(self.width, self.height)
        } else {
            self.snake.body.pop();
        }

        self.snake.head = new_head_pos;
    }

    pub fn cells(&self) -> *const CellType {
        let mut vec = Vec::<CellType>::new();

        for i in 0..self.height {
            for j in 0..self.width {
                let pos = (j, i);
                let cell_type = if pos == self.food {
                    CellType::Food
                } else if pos == self.snake.head || self.snake.body.contains(&pos) {
                    CellType::Snake
                } else if pos.0 == 0 || pos.0 == self.width - 1 || pos.1 == 0 || pos.1 == self.height - 1 {
                    CellType::Wall
                } else {
                    CellType::Empty
                };

                vec.push(cell_type)
            }
        }

        return vec.as_ptr();
    }

    fn generate_random_cell(width: u8, height: u8) -> (u8, u8) {
        let mut rng = rand::thread_rng();
        (rng.gen_range(2, width - 1) - 1, rng.gen_range(2, height - 1) - 1)
    }
}
