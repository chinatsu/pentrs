extern crate graphics;
extern crate opengl_graphics;
use graphics::*;
use piece;

pub const CELL_SIZE: f64 = 24.0;
pub const WIDTH: usize = 12;
pub const HEIGHT: usize = 25;
//pub const HIDDEN: usize = 7;


pub struct Matrix {
    pub state: [[i32; WIDTH]; HEIGHT],
    pub cleared: u64
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            state: [[0; WIDTH]; HEIGHT],
            cleared: 0
        }
    }

    pub fn clear_lines(&mut self) {
        let mut row: usize = 0;
        while row <= HEIGHT - 1 {
            let mut count = 0;
            for cell in self.state[row].iter() {
                if cell != &0 {
                    count += 1;
                }
            }
            if count == WIDTH {
                self.cleared += 1;
                for temp_row in (0..row).rev() {
                    self.state[temp_row + 1] = self.state[temp_row];
                }
                self.state[0] = [0; WIDTH]; // fill the top row with 0s i guess
            } else {
                row += 1;
            }
        }
    }

    pub fn draw(&mut self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        for y in 0..self.state.len() {
            for x in 0..self.state[y].len() {
                if self.state[y][x] != 0 {
                    let s = rectangle::square(
                        x as f64 * CELL_SIZE,
                        y as f64 * CELL_SIZE,
                        CELL_SIZE
                    );
                    rectangle(piece::get_color(self.state[y][x]), s, c.transform, gl);
                }
            }
        }
    }
}
