extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

use graphics::*;
use pieces::*;
use matrix::*;
use rand::distributions::{WeightedChoice, IndependentSample};
use std::ops;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, _rhs: Point) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, _rhs: Point) -> Point {
        Point {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y
        }
    }
}

pub struct Piece {
    pub id: i32,
    pub piece: [[Point; 5]; 4],
    pub orientation: usize,
    pub origin: Point,
    pub next_piece: [[Point; 5]; 4],
    pub next_id: i32
}

impl Piece {
    pub fn new() -> Piece {
        let mut weights = &mut WEIGHTS;
        let wc = WeightedChoice::new(weights);
        let mut rng = rand::thread_rng();
        let choice = wc.ind_sample(&mut rng);
        let next_choice = wc.ind_sample(&mut rng);
        Piece {
            id: choice as i32 + 1,
            piece: PIECES[choice],
            orientation: 0,
            origin: Point{x: 6.0, y: 1.0},
            next_piece: PIECES[next_choice],
            next_id: next_choice as i32 + 1
        }
    }

    pub fn draw(&mut self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        for i in self.piece[self.orientation].iter() {
                let x = (self.origin.x + i.x) * CELL_SIZE;
                let y = (self.origin.y + i.y) * CELL_SIZE;
                let s = rectangle::square(x, y, CELL_SIZE);
                rectangle(get_color(self.id), s, c.transform, gl);
        }
    }

    pub fn draw_next(&mut self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        let next_area = Point{x: 480.0, y: 200.0};
        for i in self.next_piece[0].iter() {
            let x = next_area.x + (i.x * CELL_SIZE);
            let y = next_area.y + (i.y * CELL_SIZE);
            let s = rectangle::square(x, y, CELL_SIZE);
            rectangle(get_color(self.next_id), s, c.transform, gl);
        }
    }

    pub fn draw_ghost(&mut self, m: &mut Matrix, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        let real_origin = self.origin;
        let mut x = self.origin.x;
        let mut y = self.origin.y;
        while self.can_move(m, Point{x: x, y: y + 1.0}) {
            x = self.origin.x;
            y = self.origin.y;
            self.move_piece(m, Point{x: 0.0, y: 1.0});
        }
        for i in self.piece[self.orientation].iter() {
                let x = (self.origin.x + i.x) * CELL_SIZE;
                let y = (self.origin.y + i.y) * CELL_SIZE;
                let s = rectangle::square(x, y, CELL_SIZE);
                rectangle([1.0, 1.0, 1.0, 0.5], s, c.transform, gl);
        }
        self.origin = real_origin;
    }

    pub fn rotate(&mut self, m: &Matrix, val: usize) {
        let new_orientation = (self.orientation + val) % self.piece.len();
        let mut new_origin = self.origin;
        if self.can_rotate(m, new_orientation, new_origin) {
            self.orientation = new_orientation;
            self.origin = new_origin;
            return;
        }
        let rotate_checks: [Point; 10] = [
            Point{x: -1.0, y: 0.0},
            Point{x: 1.0, y: 0.0},
            Point{x: 0.0, y: 1.0},
            Point{x: 1.0, y: 1.0},
            Point{x: -1.0, y: 1.0},
            Point{x: -2.0, y: 0.0},
            Point{x: 2.0, y: 0.0},
            Point{x: 0.0, y: -1.0},
            Point{x: -1.0, y: -1.0},
            Point{x: 1.0, y: -1.0}
        ];
        for check in rotate_checks.iter() {
            new_origin = self.origin + check.clone();
            if self.can_rotate(m, new_orientation, new_origin) {
                self.orientation = new_orientation;
                self.origin = new_origin;
                return;
            }
        }
    }

    fn can_rotate(&mut self, m: &Matrix, orientation: usize, origin: Point) -> bool {
        for i in self.piece[orientation].iter() {
            let x = i.x + origin.x;
            let y = i.y + origin.y;
            if x > WIDTH as f64 - 1.0 ||
            x < 0.0 ||
            y > HEIGHT as f64 - 1.0 ||
            y < 0.0 {
                return false;
            }
            if m.state[y as usize][x as usize] != 0 {
                return false;
            }
        }
        return true;
    }

    pub fn move_piece(&mut self, m: &mut Matrix, val: Point) {
        let new_origin = Point{
            x: self.origin.x + val.x,
            y: self.origin.y + val.y
        };
        if self.can_move(m, new_origin) {
            self.origin = new_origin
        }
    }

    fn can_move(&mut self, m: &mut Matrix, origin: Point) -> bool {
        for i in self.piece[self.orientation].iter() {
            let x = i.x + origin.x;
            let y = i.y + origin.y;
            if x > WIDTH as f64 - 1.0 ||
            x < 0.0 ||
            y > HEIGHT as f64 - 1.0 ||
            y < 0.0 {
                    return false;
            }
            if m.state[y as usize][x as usize] != 0 {
                return false;
            }
        }
        return true;
    }

    pub fn hard_drop(&mut self, m: &mut Matrix) {
        let mut x = self.origin.x;
        let mut y = self.origin.y;
        while self.can_move(m, Point{x: x, y: y + 1.0}) {
            x = self.origin.x;
            y = self.origin.y;
            self.move_piece(m, Point{x: 0.0, y: 1.0});
        }
        self.lock(m)
    }

    pub fn lock(&mut self, m: &mut Matrix) {
        for i in self.piece[self.orientation].iter() {
            let x = i.x + self.origin.x;
            let y = i.y + self.origin.y;
            m.state[y as usize][x as usize] = self.id;
        }
        m.clear_lines();
        self.spawn_piece();
    }

    pub fn spawn_piece(&mut self) {
        let mut weights = &mut WEIGHTS;
        let wc = WeightedChoice::new(weights);
        let mut rng = rand::thread_rng();
        let choice = wc.ind_sample(&mut rng);
        self.piece = self.next_piece;
        self.id = self.next_id;
        self.next_id = choice as i32 + 1;
        self.next_piece = PIECES[choice];
        self.origin = Point{x: 6.0, y: 1.0};
        self.orientation = 0;
    }
}

pub fn get_color(c: i32) -> [f32; 4]{
    match c {
        1 => {
            [0.941, 0.502, 0.502, 1.0]
        }
        2 => {
            [1.000, 0.412, 0.706, 1.0]
        }
        3 => {
            [1.000, 0.647, 0.000, 1.0]
        }
        4 => {
            [1.000, 0.843, 0.000, 1.0]
        }
        5 => {
            [0.941, 0.902, 0.549, 1.0]
        }
        6 => {
            [0.902, 0.902, 0.980, 1.0]
        }
        7 => {
            [0.933, 0.510, 0.933, 1.0]
        }
        8 => {
            [1.000, 0.000, 1.000, 1.0]
        }
        9 => {
            [0.502, 0.000, 0.502, 1.0]
        }
        10 => {
            [0.196, 0.804, 0.196, 1.0]
        }
        11 => {
            [0.000, 0.502, 0.000, 1.0]
        }
        12 => {
            [0.000, 1.000, 1.000, 1.0]
        }
        13 => {
            [0.255, 0.412, 0.882, 1.0]
        }
        14 => {
            [0.824, 0.706, 0.549, 1.0]
        }
        15 => {
            [0.824, 0.412, 0.118, 1.0]
        }
        16 => {
            [0.753, 0.753, 0.753, 1.0]
        }
        17 => {
            [1.000, 0.941, 0.961, 1.0]
        }
        18 => {
            [1.000, 0.871, 0.678, 1.0]
        }
        _ => {
            [0.0, 0.0, 0.0, 0.0]
        }
    }
}
