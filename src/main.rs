extern crate ggez;
extern crate ggez_test_snake;

use ggez::event;
use ggez::conf;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{DrawMode, Point2};
// use std::time::{Duration, Instant};

use ggez_test_snake::{Direction, GameState};

// mod gfx;


struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(ctx,
                         DrawMode::Fill,
                         Point2::new(self.pos_x, 380.0),
                         100.0,
                         2.0)?;
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    // c.window_title = "Rust snake test".to_string();
    c.window_width = gfx::FULL_WINDOW_SIZE.0;
    c.window_height = gfx::FULL_WINDOW_SIZE.1;
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut GameState::new().unwrap();
    event::run(ctx, state).unwrap();
}

// fn main() {
//     let snake_time_step = Duration::from_millis(120);
//     let gfx_time_step = Duration::from_millis(50);

//     let sdl_context = sdl2::init().unwrap();
//     let video = sdl_context.video().unwrap();
//     let window = video.window("SDL Test", gfx::FULL_WINDOW_SIZE.0, gfx::FULL_WINDOW_SIZE.1)
//         .position_centered()
//         .build()
//         .unwrap();
//     let mut canvas = window.into_canvas()
//         .build()
//         .unwrap();
//     let mut event_pump = sdl_context.event_pump().unwrap();

//     let mut game_state = GameState::new();
//     let mut last_move_time = Instant::now();
//     gfx::draw(&game_state, &mut canvas);
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
//             gfx::draw(&game_state, &mut canvas);
//             last_draw_time = now;
//         }
//         ::std::thread::sleep(Duration::from_millis(10));
//     }
// }
