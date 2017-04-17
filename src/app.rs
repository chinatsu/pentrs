extern crate graphics;

use opengl_graphics::glyph_cache::GlyphCache;
use opengl_graphics::GlGraphics;
use piston::input::*;
use piece::*;
use matrix::*;
use inputstate::*;

pub struct App {
    pub gl: GlGraphics,
    pub glyphs: GlyphCache<'static>,
    pub matrix: Matrix,
    pub piece: Piece,
    pub input: InputState
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let ref mut use_cache = self.glyphs;
        let ref mut inputstate = self.input;
        let ref mut piece = self.piece;
        let ref mut matrix = self.matrix;

        let mut lines: Vec<[f64; 4]> = vec![];
        let mut columns: Vec<[f64; 4]> = vec![];
        for x in 0..(args.height/32) {
            if x < (WIDTH + 1) as u32 {
                columns.push([x as f64 * CELL_SIZE, 0.0, x as f64 * CELL_SIZE, args.height as f64]);
            }
            lines.push([0.0, (x*32) as f64, WIDTH as f64 * CELL_SIZE, (x*32) as f64]);
        }

        let mut text = graphics::Text::new(22);
        text.color = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            let text_transform: graphics::context::Context= c.trans(400.0, 100.0);

            clear([0.1, 0.1, 0.1, 1.0], gl);

            for l in lines {
                line(GREEN, 0.5, l, c.transform, gl);
            }
            for col in columns {
                line(GREEN, 0.5, col, c.transform, gl);
            }

            matrix.draw(c, gl);
            let mut text_string = String::new();
            if inputstate.left {
                text_string += "left! ";
            }
            if inputstate.right {
                text_string += "right! ";
            }
            if inputstate.das_left > 0.12 || inputstate.das_right > 0.12 {
                text_string += "das!";
            }
            text.draw(&text_string, use_cache, &c.draw_state, text_transform.transform, gl);
            piece.draw_next(c, gl);
            piece.draw_ghost(matrix, c, gl);
            piece.draw(c, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if self.input.down {
            self.input.down_frames += 1;
            if self.input.down_frames % 2 == 0 {
                self.piece.move_piece(&mut self.matrix, Point{x: 0.0, y: 1.0});
            }
        }
        if self.input.das_left > 0.12 && self.input.left {
            self.piece.move_piece(&mut self.matrix, Point{x: -1.0, y: 0.0});
        }
        if self.input.das_right > 0.12 && self.input.right {
            self.piece.move_piece(&mut self.matrix, Point{x: 1.0, y: 0.0});
        }
        if self.input.left {
            if self.input.das_left == 0.0 {
                self.piece.move_piece(&mut self.matrix, Point{x: -1.0, y: 0.0});
            }
            self.input.das_left += args.dt;
        }
        if self.input.right {
            if self.input.das_right == 0.0 {
                self.piece.move_piece(&mut self.matrix, Point{x: 1.0, y: 0.0});
            }
            self.input.das_right += args.dt;
        }
    }

    pub fn on_press(&mut self, key: keyboard::Key) {
        match key {
            Key::Down => {
                self.input.down = true;
            }
            Key::Space => {
                self.piece.hard_drop(&mut self.matrix);
            }
            Key::Right => {
                self.input.right = true;
                self.input.left = false;
            }
            Key::Left => {
                self.input.left = true;
                self.input.right = false;
            }
            Key::X => {
                self.piece.rotate(&mut self.matrix, 1);
            }
            Key::Z => {
                self.piece.rotate(&mut self.matrix, 3);
            }
            Key::C => {
                self.piece.rotate(&mut self.matrix, 2);
            }
            _ => {}
        }
    }

    pub fn on_release(&mut self, key: keyboard::Key) {
        match key {
            Key::Down => {
                self.input.down = false;
                self.input.down_frames = 0;
            }
            Key::Left => {
                self.input.left = false;
                self.input.das_left = 0.0;
            }
            Key::Right => {
                self.input.right = false;
                self.input.das_right = 0.0;
            }
            _ => {}
        }
    }
}
