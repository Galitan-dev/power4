use piston_window::{rectangle, Context, G2d, line};
use piston_window::types::Color;

pub const BLOCK_SIZE: f64 = 150.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(color, [x, y, BLOCK_SIZE, BLOCK_SIZE], con.transform, g);
}

pub fn draw_line(color: Color, start_x: i32, start_y: i32, end_x: i32, end_y: i32, con: &Context, g: &mut G2d) {
    let start_x = to_coord(start_x);
    let start_y = to_coord(start_y);
    let end_x = to_coord(end_x);
    let end_y = to_coord(end_y);
    
    line(color, 10.0, [start_x, start_y, end_x, end_y], con.transform, g);
}

pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, con: &Context, g: &mut G2d) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(color, [ x, y, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)], con.transform, g)
}