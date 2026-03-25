pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}
