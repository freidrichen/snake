extern crate sdl2;
extern crate rand;

use std::time::{Duration, Instant};
use std::collections::VecDeque;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use rand::Rng;

const SQUARE_SIZE: u32 = 20;
const MAX_PLAYGROUND_HEIGHT: u32 = 40;
const MAX_PLAYGROUND_WIDTH: u32 = 50;
const FULL_WINDOW_HEIGHT: u32 = SQUARE_SIZE*MAX_PLAYGROUND_HEIGHT;
const FULL_WINDOW_WIDTH: u32 = SQUARE_SIZE*MAX_PLAYGROUND_WIDTH;


#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

type Tile = (i32, i32);

struct Level {
    width: u32,
    height: u32,
    start_tile: Tile,
    start_direction: Direction,
    barriers: Vec<Tile>,
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
            // TODO: Sanity check length of each line.
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
        level
    }
}

struct GameState {
    snake: VecDeque<Tile>,
    length: u32,
    direction: Direction,
    food: Option<Tile>,
    dead: bool,
    level: Level,
}

impl GameState {
    fn new(start_tile: Tile, direction: Direction) -> GameState {
        let mut state = GameState {
            snake: VecDeque::new(),
            length: 10,
            direction: direction,
            food: Some((5, 5)),
            dead: false,
            level: Level::from_file("resources/levels/1"),
        };
        state.snake.push_front(start_tile);
        state
    }

    fn set_direction(self: &mut GameState, direction: Direction) {
        if self.direction == Direction::Up && direction == Direction::Down
            || self.direction == Direction::Down && direction == Direction::Up
            || self.direction == Direction::Left && direction == Direction::Right
            || self.direction == Direction::Right && direction == Direction::Left {
                return
            }
        self.direction = direction;
    }

    fn move_snake(self: &mut GameState) {
        if self.dead { return; }

        let (dx, dy) = match self.direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let new_head = wraparound((
            self.snake.front().unwrap().0 + dx,
            self.snake.front().unwrap().1 + dy));
        if self.snake.contains(&new_head) {
            self.dead = true;
        } else if self.level.barriers.contains(&new_head) {
            self.dead = true;
        } else {
            self.snake.push_front(new_head);
            if self.food == Some(new_head) {
                self.length += 5;
                self.food = Some(new_food(&self.snake, &self.level.barriers));
            }
            if self.snake.len() as u32 > self.length {
                self.snake.pop_back();
            }
        }
    }
}

fn new_food(snake: &VecDeque<Tile>, barriers: &Vec<Tile>) -> Tile {
    loop {
        let tile = (rand::thread_rng().gen_range(0, MAX_PLAYGROUND_WIDTH as i32),
                    rand::thread_rng().gen_range(0, MAX_PLAYGROUND_HEIGHT as i32));
        if !barriers.contains(&tile) && !snake.contains(&tile) {
            return tile
        }
    }
}

fn wraparound(tile: Tile) -> Tile {
    let (x, y) = tile;
    let new_x = if x < 0 {
        MAX_PLAYGROUND_WIDTH as i32 - 1
    } else if x >= MAX_PLAYGROUND_WIDTH as i32 {
        0
    } else {
        x
    };
    let new_y = if y < 0 {
        MAX_PLAYGROUND_HEIGHT as i32 - 1
    } else if y >= MAX_PLAYGROUND_HEIGHT as i32 {
        0
    } else {
        y
    };
    (new_x, new_y)
}

fn to_screen_coords(tile: Tile) -> (i32, i32) {
    let (game_x, game_y) = tile;
    let screen_x = game_x*(SQUARE_SIZE as i32);
    let screen_y = game_y*(SQUARE_SIZE as i32);
    (screen_x, screen_y)
}

fn draw_tile(tile: Tile, color: Color, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(color);
    let screen_coords = to_screen_coords(tile);
    canvas.fill_rect(Rect::new(screen_coords.0, screen_coords.1,
                               SQUARE_SIZE, SQUARE_SIZE)).unwrap();
}

fn draw(game_state: &GameState, canvas: &mut Canvas<Window>) {
    let background_color = Color::RGB(0x4A, 0x99, 0x4C);
    let snake_color = Color::RGB(0xEF, 0xCD, 0x37);
    let food_color = Color::RGB(0x88, 0x2F, 0x67);
    let barrier_color = Color::RGB(0x34, 0x34, 0x34);

    canvas.set_draw_color(background_color);
    canvas.clear();

    for tile in &game_state.level.barriers {
        draw_tile(*tile, barrier_color, canvas)
    }

    for tile in &game_state.snake {
        draw_tile(*tile, snake_color, canvas)
    }

    if let Some(tile) = game_state.food {
        draw_tile(tile, food_color, canvas)
    }

    canvas.present();
}


fn main() {
    let snake_time_step = Duration::from_millis(120);
    let gfx_time_step = Duration::from_millis(50);

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video.window("SDL Test", FULL_WINDOW_WIDTH, FULL_WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas()
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut game_state = GameState::new((1, 1), Direction::Right);
    let mut last_move_time = Instant::now();
    draw(&game_state, &mut canvas);
    let mut last_draw_time = Instant::now();

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main_loop,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main_loop,
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => game_state.set_direction(Direction::Up),
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => game_state.set_direction(Direction::Down),
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => game_state.set_direction(Direction::Left),
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => game_state.set_direction(Direction::Right),
                _ => {},
            }
        }

        let now = Instant::now();
        if now.duration_since(last_move_time) > snake_time_step {
            game_state.move_snake();
            last_move_time = now;
        }

        if now.duration_since(last_draw_time) > gfx_time_step {
            draw(&game_state, &mut canvas);
            last_draw_time = now;
        }
        ::std::thread::sleep(Duration::from_millis(10));
    }
}
