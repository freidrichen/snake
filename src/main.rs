extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;
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
}

impl GameState {
    fn new(start_tile: Tile) -> GameState {
        let mut state = GameState {
            snake: VecDeque::new(),
            length: 10,
            direction: Direction::Right,
        };
        state.snake.push_front(start_tile);
        state
    }

    fn set_direction(self: &mut GameState, direction: Direction) {
        self.direction = direction;
    }

    fn move_snake(self: &mut GameState) {
        let (dx, dy) = match self.direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let new_head = (self.snake.front().unwrap().0 + dx,
                        self.snake.front().unwrap().1 + dy);
        self.snake.push_front(new_head);
        if self.snake.len() as u32 > self.length {
            self.snake.pop_back();
        }
    }


}

fn to_screen_coords(tile: Tile) -> (i32, i32) {
    let (game_x, game_y) = tile;
    let screen_x = BORDER_SIZE as i32 + game_x*(SQUARE_SIZE as i32);
    let screen_y = BORDER_SIZE as i32 + game_y*(SQUARE_SIZE as i32);
    (screen_x, screen_y)
}


fn draw(game_state: &GameState, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 210, 0));
    canvas.fill_rect(Rect::new(BORDER_SIZE as i32, BORDER_SIZE as i32,
                               PLAYGROUND_WINDOW_WIDTH, PLAYGROUND_WINDOW_HEIGHT)).unwrap();

    for tile in &game_state.snake {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let screen_coords = to_screen_coords(*tile);
        canvas.fill_rect(Rect::new(screen_coords.0, screen_coords.1,
                                   SQUARE_SIZE, SQUARE_SIZE)).unwrap();
    }

    canvas.present();
}


fn main() {
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
    let mut game_state = GameState::new((0, 0));

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main_loop,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main_loop,
                Event::KeyDown { keycode: Some(Keycode::M), .. } => game_state.move_snake(),
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => game_state.set_direction(Direction::Up),
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => game_state.set_direction(Direction::Down),
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => game_state.set_direction(Direction::Left),
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => game_state.set_direction(Direction::Right),
                _ => {},
            }
        }

        draw(&game_state, &mut canvas);
        ::std::thread::sleep(Duration::from_millis(50));
    }
}
