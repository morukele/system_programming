use crossbeam::unbounded;
use rayon::prelude::*;
use std::thread;
use svg::node::element::path::{Command, Data, Position};
use svg::node::element::{Path, Rectangle};
use svg::Document;

// Declaring the boundary of the SVG image
const WIDTH: isize = 400;
const HEIGHT: isize = WIDTH;

// HOME is synonymous for center and is calculated based off the height and width
const HOME_Y: isize = HEIGHT / 2;
const HOME_X: isize = WIDTH / 2;

// This is the max size of the lines in the SVG image
const STROKE_WIDTH: usize = 5;
#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Nope(u8),
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    North,
    East,
    West,
    South,
}

#[derive(Debug)]
struct Artist {
    x: isize,
    y: isize,
    heading: Orientation,
}

enum Work {
    Task((usize, u8)),
    Finished,
}

impl Artist {
    fn new() -> Artist {
        Artist {
            heading: Orientation::North,
            x: HOME_X,
            y: HOME_Y,
        }
    }

    fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }

    fn forward(&mut self, distance: isize) {
        match self.heading {
            Orientation::North => self.y += distance,
            Orientation::South => self.y -= distance,
            Orientation::West => self.x += distance,
            Orientation::East => self.x -= distance,
        };
    }

    fn turn_right(&mut self) {
        self.heading = match self.heading {
            Orientation::North => Orientation::East,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
            Orientation::East => Orientation::South,
        }
    }

    fn turn_left(&mut self) {
        self.heading = match self.heading {
            Orientation::North => Orientation::West,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South,
            Orientation::East => Orientation::North,
        }
    }

    fn wrap(&mut self) {
        if self.x < 0 {
            self.x = HOME_X;
            self.heading = Orientation::West;
        } else if self.x > WIDTH {
            self.x = HOME_X;
            self.heading = Orientation::East;
        }

        if self.y < 0 {
            self.y = HOME_Y;
            self.heading = Orientation::North;
        } else if self.y > HEIGHT {
            self.y = HOME_Y;
            self.heading = Orientation::South;
        }
    }
}

fn parse_byte(byte: u8) -> Operation {
    match byte {
        b'0' => Operation::Home,
        b'1'..=b'9' => {
            let distance = (byte - 0x30) as isize;
            Operation::Forward(distance * (HEIGHT / 10))
        }
        b'a' | b'b' | b'c' => Operation::TurnLeft,
        b'd' | b'e' | b'f' => Operation::TurnRight,
        _ => Operation::Nope(byte),
    }
}

pub fn parse(input: &str) -> Vec<Operation> {
    let n_threads = 2;
    let (todo_tx, todo_rx) = unbounded();
    let (results_tx, results_rx) = unbounded();
    let mut n_bytes = 0;
    for (i, byte) in input.bytes().enumerate() {
        todo_tx.send(Work::Task((i, byte))).unwrap();
        n_bytes += 1;
    }

    for _ in 0..n_threads {
        todo_tx.send(Work::Finished).unwrap();
    }

    for _ in 0..n_threads {
        let todo = todo_rx.clone();
        let results = results_tx.clone();
        thread::spawn(move || loop {
            let task = todo.recv();
            let result = match task {
                Err(_) => break,
                Ok(Work::Finished) => break,
                Ok(Work::Task((i, byte))) => (i, parse_byte(byte)),
            };
            results.send(result).unwrap();
        });
    }
    let mut ops = vec![Operation::Nope(0); n_bytes];
    for _ in 0..n_bytes {
        let (i, op) = results_rx.recv().unwrap();
        ops[i] = op;
    }

    ops
}

pub fn convert(operations: &Vec<Operation>) -> Vec<Command> {
    let mut turtle = Artist::new();

    let mut path_data = Vec::<Command>::with_capacity(operations.len());
    let start_at_home = Command::Move(Position::Absolute, (HOME_X, HOME_Y).into());
    path_data.push(start_at_home);

    for op in operations {
        match *op {
            Operation::Forward(distance) => turtle.forward(distance),
            Operation::TurnLeft => turtle.turn_left(),
            Operation::TurnRight => turtle.turn_right(),
            Operation::Home => turtle.home(),
            Operation::Nope(byte) => {
                eprintln!("warning: illegal byte encountered: {:?}", byte);
            }
        };

        let path_segment = Command::Line(Position::Absolute, (turtle.x, turtle.y).into());
        path_data.push(path_segment);
        turtle.wrap();
    }

    path_data
}

pub fn generate_svg(path_data: Vec<Command>) -> Document {
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .set("fill", "#ffffff");

    let border = background
        .clone()
        .set("fill-opacity", "0.0")
        .set("stroke", "#cccccc")
        .set("stroke-width", 3 * STROKE_WIDTH);

    let sketch = Path::new()
        .set("fill", "none")
        .set("stroke", "#2f2f2f")
        .set("stroke-width", STROKE_WIDTH)
        .set("stroke-opacity", "0.9")
        .set("d", Data::from(path_data));

    Document::new()
        .set("viewBox", (0, 0, HEIGHT, WIDTH))
        .set("height", HEIGHT)
        .set("width", WIDTH)
        .set("style", "style=\"outline: 5px solid #800000;\"")
        .add(background)
        .add(sketch)
        .add(border)
}
