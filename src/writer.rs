use crate::constants::{FRAME_HEIGHT, FRAME_WIDTH};
use crate::structs::Point;

pub fn set_pixel(render_data: &mut [char], x: usize, y: usize, ch: char) {
    if x >= FRAME_WIDTH || y >= FRAME_HEIGHT {
        println!("x must be less that FRAME_WIDTH, y must be less than FRAME_HEIGHT");
    } else {
        render_data[y * FRAME_WIDTH + x] = ch;
    }
}

pub fn draw_line(point0: &Point, point1: &Point, render_data: &mut [char]) {
    let mut x0 = ((point0.x + 1.0) * 0.5 * FRAME_WIDTH as f32).round() as isize;
    let mut y0 = ((point0.y + 1.0) * 0.5 * FRAME_HEIGHT as f32).round() as isize;
    let x1 = ((point1.x + 1.0) * 0.5 * FRAME_WIDTH as f32).round() as isize;
    let y1 = ((point1.y + 1.0) * 0.5 * FRAME_HEIGHT as f32).round() as isize;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;
    loop {
        set_pixel(render_data, x0 as usize, y0 as usize, '#');
        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

pub fn draw(
    vertices: Vec<Point>,
    edges: Vec<(usize, usize)>,
    render_data: &mut [char],
) -> Option<()> {
    for edge in edges {
        let point0 = vertices.get(edge.0)?;
        let point1 = vertices.get(edge.1)?;
        draw_line(point0, point1, render_data);
    }
    Some(())
}
