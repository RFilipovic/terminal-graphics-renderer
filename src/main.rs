use crossterm::terminal::enable_raw_mode;
use std::io::{self, Write};

mod constants;
mod helpers;
mod input_handler;
mod perspective_projection;
mod rotation;
mod structs;
mod writer;

use constants::{FRAME_HEIGHT, FRAME_WIDTH};
use helpers::serialize;
use structs::*;

use crate::helpers::transform_vec3_to_base;
use crate::input_handler::handle_input;
use crate::perspective_projection::perspective_projection;
use crate::rotation::rotate_point;
use crate::writer::draw;

fn main() -> Result<(), std::io::Error> {
    enable_raw_mode().unwrap();

    let mut out = io::stdout();
    let fb = FrameBuffer::new(FRAME_WIDTH, FRAME_HEIGHT);
    let mut render_data: Vec<char> = vec![' '; fb.width * fb.height];

    let mut cube_vertices = vec![
        Vec3 {
            x: -1.0,
            y: -1.0,
            z: -1.0,
        },
        Vec3 {
            x: 1.0,
            y: -1.0,
            z: -1.0,
        },
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: -1.0,
        },
        Vec3 {
            x: -1.0,
            y: 1.0,
            z: -1.0,
        },
        Vec3 {
            x: -1.0,
            y: -1.0,
            z: 1.0,
        },
        Vec3 {
            x: 1.0,
            y: -1.0,
            z: 1.0,
        },
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        Vec3 {
            x: -1.0,
            y: 1.0,
            z: 1.0,
        },
    ];

    transform_vec3_to_base(&mut cube_vertices);

    let cube_edges = vec![
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];

    loop {
        if let Some((axis, direction)) = handle_input() {
            rotate_point(&mut cube_vertices, &axis, direction);
        }
        render_data.fill(' ');
        draw(
            perspective_projection(&cube_vertices),
            &cube_edges,
            &mut render_data,
        );
        write!(out, "\x1b[2J\x1b[H")?;
        out.flush()?;
        println!("{}", serialize(&render_data, FRAME_WIDTH, FRAME_HEIGHT));
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
