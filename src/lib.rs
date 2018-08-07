extern crate rand;

use std::collections::VecDeque;
use std::io::prelude::*;
use std::fs::File;
use rand::Rng;

pub const MAX_PLAYGROUND_SIZE: (u32, u32) = (50, 40);


#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn is_opposite(self: Direction, other: Direction) -> bool {
        (self == Direction::Up && other == Direction::Down
         || self == Direction::Down && other == Direction::Up
         || self == Direction::Left && other == Direction::Right
         || self == Direction::Right && other == Direction::Left)
    }
}

pub type Tile = (i32, i32);

pub struct Level {
    pub width: u32,
    pub height: u32,
    start_tile: Tile,
    start_direction: Direction,
    pub barriers: Vec<Tile>,
}

impl Level {
    fn from_file(filename: &str) -> Level {
        let mut f = File::open(filename).expect("Error opening level file.");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Error reading level file.");
        let mut level = Level {
            width: 0,
            height: 0,
            start_tile: (0, 0),
            start_direction: Direction::Right,
            barriers: vec![]
        };
        for (y, line) in contents.lines().enumerate() {
            if level.width > 0 && line.len() as u32 != level.width {
                panic!("Multiple widths in level file!")
            }
            else if line.len() as u32 > MAX_PLAYGROUND_SIZE.0 {
                panic!("Level is too wide!")
            }
            level.width = line.len() as u32;
            level.height = y as u32;
            for (x, c) in line.chars().enumerate() {
                let tile = (x as i32, y as i32);
                match c {
                    '#' => level.barriers.push(tile),
                    '.' | ' ' => continue,
                    '<' => {
                        level.start_tile = tile;
                        level.start_direction = Direction::Left;
                    },
                    '>' => {
                        level.start_tile = tile;
                        level.start_direction = Direction::Right;
                    },
                    '^' => {
                        level.start_tile = tile;
                        level.start_direction = Direction::Up;
                    },
                    'v' => {
                        level.start_tile = tile;
                        level.start_direction = Direction::Down;
                    },
                    _ => {
                        panic!("Invalid characters in level file: {:?}.", c)
                    }
                }
            }
        }
        level.height += 1;
        if level.height > MAX_PLAYGROUND_SIZE.1 {
            panic!("Level is too high!")
        }
        level
    }

    fn wraparound(self: &Level, tile: Tile) -> Tile {
        let (x, y) = tile;
        let new_x = if x < 0 {
            self.width as i32 - 1
        } else if x >= self.width as i32 {
            0
        } else {
            x
        };
        let new_y = if y < 0 {
            self.height as i32 - 1
        } else if y >= self.height as i32 {
            0
        } else {
            y
        };
        (new_x, new_y)
    }
}

pub struct GameState {
    pub score: u32,
    pub snake: VecDeque<Tile>,
    length: u32,
    direction: Direction,
    future_directions: VecDeque<Direction>,
    pub food: Option<Tile>,
    gameover: bool,
    pub level: Level,
}

impl GameState {
    pub fn new() -> GameState {
        let level = Level::from_file("resources/levels/1");
        let mut snake = VecDeque::new();
        let food = new_food(&snake, &level);
        snake.push_front(level.start_tile);
        GameState {
            score: 0,
            snake: snake,
            length: 10,
            direction: level.start_direction,
            future_directions: VecDeque::new(),
            food: Some(food),
            gameover: false,
            level: level,
        }
    }

    pub fn set_direction(self: &mut GameState, direction: Direction) {
        self.future_directions.push_back(direction);
    }

    pub fn update(self: &mut GameState) {
        if self.gameover { return; }

        if let Some(direction) = self.future_directions.pop_front() {
            if !self.direction.is_opposite(direction) {
                self.direction = direction;
            }
        }
        let (dx, dy) = match self.direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let new_head = self.level.wraparound((
            self.snake.front().unwrap().0 + dx,
            self.snake.front().unwrap().1 + dy));
        if self.snake.contains(&new_head) {
            self.gameover = true;
        } else if self.level.barriers.contains(&new_head) {
            self.gameover = true;
        } else {
            self.snake.push_front(new_head);
            if self.food == Some(new_head) {
                self.score += 1;
                println!("Score: {}", self.score);
                self.length += 5;
                self.food = Some(new_food(&self.snake, &self.level));
            }
            if self.snake.len() as u32 > self.length {
                self.snake.pop_back();
            }
        }
    }
}

fn new_food(snake: &VecDeque<Tile>, level: &Level) -> Tile {
    loop {
        let tile = (rand::thread_rng().gen_range(0, level.width as i32),
                    rand::thread_rng().gen_range(0, level.height as i32));
        if !level.barriers.contains(&tile) && !snake.contains(&tile) {
            return tile
        }
    }
}
