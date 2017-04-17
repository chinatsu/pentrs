extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate find_folder;
extern crate rand;

use piston::window::*;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::GlyphCache;

mod pieces;
mod piece;
mod matrix;
mod inputstate;
mod app;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "pentrs",
        [((matrix::WIDTH*2)*matrix::CELL_SIZE as usize) as u32, (matrix::HEIGHT*matrix::CELL_SIZE as usize) as u32]
    )
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets").unwrap();
    let ref font = assets.join("FiraMono-Regular.ttf");

    let mut game = app::App {
        gl: GlGraphics::new(opengl),
        glyphs: GlyphCache::new(font).unwrap(),
        piece: piece::Piece::new(),
        matrix: matrix::Matrix::new(),
        input: inputstate::InputState::new()
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.on_press(key);
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            game.on_release(key);
        }
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
        if let Some(u) = e.update_args() {
            game.update(&u);
        }
    }
}
