use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::env;
use std::io::{self, Write};

mod constants;
mod helpers;
mod input_handler;
mod parse_object;
mod perspective_projection;
mod rotation;
mod structs;
mod writer;

use constants::{FRAME_HEIGHT, FRAME_WIDTH};
use helpers::serialize;
use structs::*;

use crate::helpers::transform_vec3_to_base;
use crate::input_handler::handle_input;
use crate::parse_object::{parse_lines, read_file};
use crate::perspective_projection::perspective_projection;
use crate::rotation::rotate_point;
use crate::writer::draw;

fn main() -> Result<(), std::io::Error> {
    enable_raw_mode().unwrap();

    let mut out = io::stdout();
    let fb = FrameBuffer::new(FRAME_WIDTH, FRAME_HEIGHT);
    let mut render_data: Vec<char> = vec![' '; fb.width * fb.height];

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(0);
    } else if args.len() > 2 {
        eprintln!("Program takes only one argument - file_name");
        std::process::exit(0);
    }

    let file_name = &args[1];

    let lines = read_file(file_name).expect("Failed to read from file.");
    let (mut vertices, mut edges) = parse_lines(lines);

    transform_vec3_to_base(&mut vertices);

    loop {
        if let Some((axis, direction)) = handle_input() {
            rotate_point(&mut vertices, &axis, direction);
        }
        render_data.fill(' ');
        draw(perspective_projection(&vertices), &edges, &mut render_data);
        write!(out, "\x1b[2J\x1b[H")?;
        out.flush()?;
        println!("{}", serialize(&render_data, FRAME_WIDTH, FRAME_HEIGHT));
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    disable_raw_mode().unwrap();
    Ok(())
}
