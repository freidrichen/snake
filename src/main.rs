extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::{Duration, Instant};
use std::collections::VecDeque;


const SQUARE_SIZE: u32 = 20;
const BORDER_SIZE: u32 = 10;
const PLAYGROUND_HEIGHT: u32 = 40;
const PLAYGROUND_WIDTH: u32 = 50;
const PLAYGROUND_WINDOW_HEIGHT: u32 = SQUARE_SIZE*PLAYGROUND_HEIGHT;
const PLAYGROUND_WINDOW_WIDTH: u32 = SQUARE_SIZE*PLAYGROUND_WIDTH;
const FULL_WINDOW_HEIGHT: u32 = SQUARE_SIZE*PLAYGROUND_HEIGHT + 2*BORDER_SIZE;
const FULL_WINDOW_WIDTH: u32 = SQUARE_SIZE*PLAYGROUND_WIDTH + 2*BORDER_SIZE;


enum Direction {
    Up,
    Down,
    Left,
    Right
}

type Tile = (i32, i32);
struct GameState {
    snake: VecDeque<Tile>,
    length: u32,
    direction: Direction,
    food: Option<Tile>,
    dead: bool,
}

impl GameState {
    fn new(start_tile: Tile, direction: Direction) -> GameState {
        let mut state = GameState {
            snake: VecDeque::new(),
            length: 10,
            direction: direction,
            food: Some((5, 5)),
            dead: false,
        };
        state.snake.push_front(start_tile);
        state
    }

    fn set_direction(self: &mut GameState, direction: Direction) {
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
        let new_head = (self.snake.front().unwrap().0 + dx,
                        self.snake.front().unwrap().1 + dy);
        if self.snake.contains(&new_head) {
            self.dead = true;
        } else {
            self.snake.push_front(new_head);
            if self.food == Some(new_head) {
                self.length += 3;
                self.food = None;
            }
            if self.snake.len() as u32 > self.length {
                self.snake.pop_back();
            }
        }
    }


}

fn to_screen_coords(tile: Tile) -> (i32, i32) {
    let (game_x, game_y) = tile;
    let screen_x = BORDER_SIZE as i32 + game_x*(SQUARE_SIZE as i32);
    let screen_y = BORDER_SIZE as i32 + game_y*(SQUARE_SIZE as i32);
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

    canvas.set_draw_color(barrier_color);
    canvas.clear();

    canvas.set_draw_color(background_color);
    canvas.fill_rect(Rect::new(BORDER_SIZE as i32, BORDER_SIZE as i32,
                               PLAYGROUND_WINDOW_WIDTH, PLAYGROUND_WINDOW_HEIGHT)).unwrap();

    for tile in &game_state.snake {
        draw_tile(*tile, snake_color, canvas)
    }

    if let Some(tile) = game_state.food {
        draw_tile(tile, food_color, canvas)
    }

    canvas.present();
}


fn main() {
    let snake_time_step = Duration::from_millis(150);
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
