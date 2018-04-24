extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;


const SQUARE_SIZE: u32 = 20;
const BORDER_SIZE: u32 = 10;
const PLAYGROUND_HEIGHT: u32 = 40;
const PLAYGROUND_WIDTH: u32 = 50;
const PLAYGROUND_WINDOW_HEIGHT: u32 = SQUARE_SIZE*PLAYGROUND_HEIGHT;
const PLAYGROUND_WINDOW_WIDTH: u32 = SQUARE_SIZE*PLAYGROUND_WIDTH;
const FULL_WINDOW_HEIGHT: u32 = SQUARE_SIZE*PLAYGROUND_HEIGHT + 2*BORDER_SIZE;
const FULL_WINDOW_WIDTH: u32 = SQUARE_SIZE*PLAYGROUND_WIDTH + 2*BORDER_SIZE;


type GameState = (i32, i32);


fn to_screen_coords(game_x: i32, game_y: i32) -> (i32, i32) {
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

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let screen_coords = to_screen_coords(game_state.0, game_state.1);
    canvas.fill_rect(Rect::new(screen_coords.0, screen_coords.1,
                               SQUARE_SIZE, SQUARE_SIZE)).unwrap();

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
    let mut game_state = (0, 0);

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main_loop,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main_loop,
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => game_state.1 -= 1,
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => game_state.1 += 1,
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => game_state.0 -= 1,
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => game_state.0 += 1,
                _ => {},
            }
        }

        draw(&game_state, &mut canvas);
        ::std::thread::sleep(Duration::from_millis(50));
    }
}
