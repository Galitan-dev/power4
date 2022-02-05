extern crate piston_window;

mod draw;
mod game;
mod grid;

use piston_window::*;
use piston_window::types::Color;

use grid::{COLUMNS, ROWS};
use draw::{to_coord_u32};
use game::Game;

const BACKGROUND_COLOR: Color = [0.2, 0.2, 0.2, 1.0];

fn main() {
    
    let (width, height) = (COLUMNS as i32, ROWS as i32);

    let mut window: PistonWindow = WindowSettings::new("Power 4", [ to_coord_u32(width), to_coord_u32(height) ])
        .exit_on_esc(true)
        .resizable(false)
        .build().unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BACKGROUND_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });

    }

}
