extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video.window("SDL Test", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 210, 0));
    canvas.fill_rect(Rect::new(10, 10, 780, 580)).unwrap();

    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut game_state = 

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main_loop,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main_loop,
                _ => {},
            }
        }

        ::std::thread::sleep(Duration::from_millis(50));
    }
}
