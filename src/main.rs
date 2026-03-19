use std::io::{self, Write};
use std::thread;
use std::time::Duration;

mod constants;
mod helpers;
mod perspective_projection;
mod structs;
mod writer;

use constants::{FRAME_HEIGHT, FRAME_WIDTH};
use helpers::serialize;
use structs::*;

use crate::helpers::transform_vec3_to_base;
use crate::perspective_projection::perspective_projection;
use crate::writer::draw;

fn main() -> Result<(), std::io::Error> {
    //let mut out = io::stdout();
    let fb = FrameBuffer::new(FRAME_WIDTH, FRAME_HEIGHT);

    let mut render_data: Vec<char> = vec![' '; fb.width * fb.height];
    /*
        loop {
            write!(out, "\x1b[2J")?;
            out.flush()?;

            write!(out, "\x1b[H")?;
            out.flush()?;

            println!("{}", serialize(&render_data, FRAME_WIDTH, FRAME_HEIGHT));
        }
    */

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
    let vertices = perspective_projection(cube_vertices);

    let cube_edges = vec![
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0), // bottom face
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4), // top face
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7), // vertical edges
    ];

    draw(vertices, cube_edges, &mut render_data);
    println!("{}", serialize(&render_data, FRAME_WIDTH, FRAME_HEIGHT));

    Ok(())
}
