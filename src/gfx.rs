extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2_test_snake::MAX_PLAYGROUND_SIZE;
use sdl2_test_snake::{Tile, GameState};

const SQUARE_SIZE: u32 = 20;
pub const FULL_WINDOW_SIZE: (u32, u32) = (
    SQUARE_SIZE*MAX_PLAYGROUND_SIZE.0,
    SQUARE_SIZE*MAX_PLAYGROUND_SIZE.1);


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

pub fn draw(game_state: &GameState, canvas: &mut Canvas<Window>) {
    let outside_color = Color::RGB(0xCA, 0xC5, 0xAE);
    let background_color = Color::RGB(0x4A, 0x99, 0x4C);
    let snake_color = Color::RGB(0xEF, 0xCD, 0x37);
    let food_color = Color::RGB(0x88, 0x2F, 0x67);
    let barrier_color = Color::RGB(0x34, 0x34, 0x34);

    canvas.set_draw_color(outside_color);
    canvas.clear();

    canvas.set_draw_color(background_color);
    let height = SQUARE_SIZE*game_state.level.height;
    let width = SQUARE_SIZE*game_state.level.width;
    canvas.fill_rect(Rect::new(0, 1, width, height)).unwrap();

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
