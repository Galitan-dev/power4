use piston_window::*;
use piston_window::types::Color;

use crate::draw::{draw_rectangle};
use crate::grid::{MyGrid, COLUMNS, ROWS};

const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];

pub struct Game {
    grid: MyGrid,

    width: i32,
    height: i32,

    current_player: i8,

    game_over: bool,
    waiting_time: f64
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            grid: MyGrid::new(),
            waiting_time: 0.0,
            current_player: 0,
            width,
            height,
            game_over: false
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.grid.draw(self.current_player, con, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.grid.selector().is_none() {
            for i in 0..COLUMNS {
                let j = self.grid.get_column_length(i as i32);

                if j < ROWS as i32 {
                    self.grid.select(i as i32);
                    break;
                }
            }

            self.current_player = (self.current_player + 1) % 2;
        }

        if self.game_over {
            return;
        }

    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.grid.selector().is_none() {
            return;
        }

        match key {
            Key::Left => self.grid.move_selector(-1),
            Key::Right => self.grid.move_selector(1),
            Key::Return => {
                if let Some((x, y)) = self.grid.place(self.current_player) {
                    self.check_gameover(x, y);
                }
            },
            _ => (),
        }
    }

    fn check_gameover(&self, x: i32, y: i32) {
        
    }
}