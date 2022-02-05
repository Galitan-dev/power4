use array_init::array_init;
use piston_window::*;
use piston_window::types::Color;

use crate::draw::{draw_line, draw_block};

pub const COLUMNS: usize = 7;
pub const ROWS: usize = 6;

const PLAYER_COLORS: [Color; 2] = [[0.8, 0.0, 0.0, 1.0], [0.0, 0.8, 0.0, 1.0]];
const BORDER_COLOR: Color = [0.6, 0.6, 0.6, 1.0];

#[derive(Copy, Clone)]
pub struct Cell {
    player: Option<i8>
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            player: None
        }
    }

    pub fn get_player(&self) -> Option<i8> {
        self.player
    }

    pub fn set_player(&mut self, player: i8) {
        self.player = Some(player);
    }
}

pub struct MyGrid {
    cells: [[Cell;ROWS];COLUMNS],
    column_selector: Option<i32>
}

impl MyGrid {
    pub fn new() -> MyGrid {
        MyGrid {
            cells: [[Cell::new(); ROWS]; COLUMNS],
            column_selector: None
        }
    }

    pub fn draw(&self, current_player: i8, con: &Context, g: &mut G2d) {
        if self.column_selector.is_some() {
            let y = self.get_column_length(self.column_selector.unwrap());

            let mut color = PLAYER_COLORS[current_player as usize];
            color[3] = 0.3;

            draw_block(
                color, 
                self.column_selector.unwrap(), ROWS as i32 - y - 1, 
                con, g
            );
        }

        for (x, y, cell) in self.iter() {
            if cell.player.is_some() {
                draw_block(
                    PLAYER_COLORS[cell.player.unwrap() as usize], 
                    x, ROWS as i32 - y - 1, 
                    con, g
                );
            }
        }

        draw_line(BORDER_COLOR, 0, 0, COLUMNS as i32, 0, con, g);
        draw_line(BORDER_COLOR, 0, ROWS as i32, COLUMNS as i32, ROWS as i32, con, g);
        draw_line(BORDER_COLOR, 0, 0, 0, ROWS as i32, con, g);
        draw_line(BORDER_COLOR, COLUMNS as i32, 0, COLUMNS as i32, ROWS as i32, con, g);

        for i in 1..COLUMNS as i32 {
            draw_line(BORDER_COLOR, i, 0, i, ROWS as i32, con, g);
        }

        for i in 0..ROWS as i32 {
            draw_line(BORDER_COLOR, 0, i, COLUMNS as i32, i, con, g);
        }
    }

    pub fn get_column(&self, column: i32) -> [Cell; ROWS] {
        self.cells[column as usize]
    }

    pub fn get_column_length(&self, column: i32) -> i32 {
        let mut y: i32 = 0;
        for cell in self.get_column(column) {
            match cell.get_player() {
                Some(_) => y += 1,
                None => (),
            }
        }

        y
    }

    pub fn iter(&self) -> [(i32, i32, Cell); COLUMNS * ROWS] {
        array_init(|i| {
            let x = (i as f32 / ROWS as f32).floor() as i32;
            let y = i as i32 % (ROWS as i32);
            (x, y, self.cells[x as usize][y as usize])
        })
    }

    fn set_player(&mut self, x: i32, y: i32, player: i8) {
        self.cells[x as usize][y as usize].set_player(player);
    }

    pub fn selector(&self) -> Option<i32> {
        self.column_selector
    }

    pub fn select(&mut self, column: i32) {
        if column >= 0 && column < COLUMNS as i32 {
            self.column_selector = Some(column);
        }
    }

    pub fn move_selector(&mut self, dir: i32) {
        if self.column_selector.is_none() {
            return;
        }

        for i in 1..COLUMNS as i32 {
            let x = self.column_selector.unwrap() + i * dir;
            if x >= 0 && x < COLUMNS as i32 && self.get_column_length(x) < ROWS as i32 {
                self.column_selector = Some(x);
                break;
            }
        }
    }

    pub fn place(&mut self, player: i8) -> Option<(i32, i32)> {
        if self.column_selector.is_none() {
            return None;
        }

        let selector = self.column_selector.unwrap();
        self.column_selector = None;

        let y = self.get_column_length(selector);
        self.set_player(selector, y, player);

        Some((selector, y))
    }
}