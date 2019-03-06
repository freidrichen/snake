#![feature(duration_float)]
extern crate rand;
extern crate ggez;

use ggez::{Context, GameResult};
use ggez::event::{EventHandler};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::graphics::{self, Rect, Color};
use std::collections::VecDeque;
use std::io::prelude::*;
use std::fs::File;
use std::time::{Instant, Duration};
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
    fn from_file(filename: &str) -> GameResult<Level> {
        let mut f = File::open(filename)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
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
        Ok(level)
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
    step_delay: Duration,
    last_step: Instant,
}

impl GameState {
    pub fn new() -> GameResult<GameState> {
        let level = Level::from_file("resources/levels/1")?;
        let mut snake = VecDeque::new();
        let food = new_food(&snake, &level);
        snake.push_front(level.start_tile);
        Ok(GameState {
            score: 0,
            snake: snake,
            length: 10,
            direction: level.start_direction,
            future_directions: VecDeque::new(),
            food: Some(food),
            gameover: false,
            level: level,
            step_delay: Duration::from_millis(162),
            last_step: Instant::now(),
        })
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.future_directions.push_back(direction);
    }

    fn update_single_step(&mut self) {
        if self.gameover { return }

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
                self.step_delay = self.step_delay.mul_f64(0.95);
                println!("Score: {}; Speed: {:?}", self.score, self.step_delay);
                self.length += 5;
                self.food = Some(new_food(&self.snake, &self.level));
            }
            if self.snake.len() as u32 > self.length {
                self.snake.pop_back();
            }
        }
    }
}

impl EventHandler for GameState {
    fn draw(self: &mut GameState, context: &mut Context) -> GameResult<()> {
        draw(self, context)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn update(self: &mut GameState, _ctx: &mut Context) -> GameResult<()> {
        let mut dt = Instant::now() - self.last_step;
        while dt >= self.step_delay {
            dt -= self.step_delay;
            self.update_single_step();
            // If we updated, we set our last_update to the time at which the
            // update took place.
            self.last_step = Instant::now() - dt;
        }
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode,
                      _keymod: KeyMods, _repeat: bool) {
        match keycode {
            // Event::Quit {..} => break 'main_loop,
            // Event::KeyDown { keycode: Some(KeyCode::Escape), .. } => break 'main_loop,
            KeyCode::Up => self.set_direction(Direction::Up),
            KeyCode::Down => self.set_direction(Direction::Down),
            KeyCode::Left => self.set_direction(Direction::Left),
            KeyCode::Right => self.set_direction(Direction::Right),
            _ => {},
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

const SQUARE_SIZE: u32 = 20;
pub const FULL_WINDOW_SIZE: (u32, u32) = (
    SQUARE_SIZE*MAX_PLAYGROUND_SIZE.0,
    SQUARE_SIZE*MAX_PLAYGROUND_SIZE.1);


fn to_screen_coords(tile: Tile) -> (f32, f32) {
    let (game_x, game_y) = tile;
    let screen_x = (game_x as f32)*(SQUARE_SIZE as f32);
    let screen_y = (game_y as f32)*(SQUARE_SIZE as f32);
    (screen_x, screen_y)
}

fn draw_tile(ctx: &mut Context, tile: Tile, color: Color) -> GameResult<()> {
    let screen_coords = to_screen_coords(tile);

    let size_rect = Rect::new(screen_coords.0, screen_coords.1,
                              SQUARE_SIZE as f32, SQUARE_SIZE as f32);
    let mesh_rect = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        size_rect,
        color)?;
    graphics::draw(ctx, &mesh_rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

    Ok(())
}

pub fn draw(game_state: &GameState, ctx: &mut Context) -> GameResult<()> {
    let outside_color = Color::from_rgb(0xCA, 0xC5, 0xAE);
    let background_color = Color::from_rgb(0x4A, 0x99, 0x4C);
    let snake_color = Color::from_rgb(0xEF, 0xCD, 0x37);
    let food_color = Color::from_rgb(0x88, 0x2F, 0x67);
    let barrier_color = Color::from_rgb(0x34, 0x34, 0x34);

    graphics::clear(ctx, outside_color);

    let height = (SQUARE_SIZE*game_state.level.height) as f32;
    let width = (SQUARE_SIZE*game_state.level.width) as f32;
    let bg_rect = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        Rect::new(0.0, 1.0, width, height),
        background_color)?;
    graphics::draw(ctx, &bg_rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

    for tile in &game_state.level.barriers {
        draw_tile(ctx, *tile, barrier_color)?
    }

    for tile in &game_state.snake {
        draw_tile(ctx, *tile, snake_color)?
    }

    if let Some(tile) = game_state.food {
        draw_tile(ctx, tile, food_color)?
    }

    graphics::present(ctx)?;
    Ok(())
}
