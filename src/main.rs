extern crate ggez;
extern crate snake;

use ggez::event;
use ggez::GameResult;
// use std::time::{Duration, Instant};

// use snake::{Direction, GameState};
use snake::GameState;

const MAX_PLAYGROUND_SIZE: (u32, u32) = (50, 40);
const SQUARE_SIZE: u32 = 20;
pub const FULL_WINDOW_SIZE: (u32, u32) = (
    SQUARE_SIZE*MAX_PLAYGROUND_SIZE.0,
    SQUARE_SIZE*MAX_PLAYGROUND_SIZE.1);


pub fn main() -> GameResult {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("snake", "Magnus Sandén")
        .window_setup(ggez::conf::WindowSetup::default().title("Rusty McSnake"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(FULL_WINDOW_SIZE.0 as f32, FULL_WINDOW_SIZE.1 as f32))
        .build()?;

    let state = &mut GameState::new().unwrap();
    event::run(ctx, event_loop, state).unwrap();
    Ok(())
}

// fn main() {
//     let snake_time_step = Duration::from_millis(120);
//     let gfx_time_step = Duration::from_millis(50);

//     let sdl_context = sdl2::init().unwrap();
//     let video = sdl_context.video().unwrap();
//     let window = video.window("SDL Test", FULL_WINDOW_SIZE.0, FULL_WINDOW_SIZE.1)
//         .position_centered()
//         .build()
//         .unwrap();
//     let mut canvas = window.into_canvas()
//         .build()
//         .unwrap();
//     let mut event_pump = sdl_context.event_pump().unwrap();

//     let mut game_state = GameState::new();
//     let mut last_move_time = Instant::now();
//     draw(&game_state, &mut canvas);
//     let mut last_draw_time = Instant::now();

//     'main_loop: loop {
//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit {..} => break 'main_loop,
//                 Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main_loop,
//                 Event::KeyDown { keycode: Some(Keycode::Up), .. } => game_state.set_direction(Direction::Up),
//                 Event::KeyDown { keycode: Some(Keycode::Down), .. } => game_state.set_direction(Direction::Down),
//                 Event::KeyDown { keycode: Some(Keycode::Left), .. } => game_state.set_direction(Direction::Left),
//                 Event::KeyDown { keycode: Some(Keycode::Right), .. } => game_state.set_direction(Direction::Right),
//                 _ => {},
//             }
//         }

//         let now = Instant::now();
//         if now.duration_since(last_move_time) > snake_time_step {
//             game_state.update();
//             last_move_time = now;
//         }

//         if now.duration_since(last_draw_time) > gfx_time_step {
//             draw(&game_state, &mut canvas);
//             last_draw_time = now;
//         }
//         ::std::thread::sleep(Duration::from_millis(10));
//     }
// }
