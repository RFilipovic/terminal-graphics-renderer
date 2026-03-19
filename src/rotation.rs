use crate::constants::THETA;
use crate::structs::Vec3;

pub fn rotate_point(vertices: &mut Vec<Vec3>, axis: &char, direction: f32) {
    let theta = THETA * direction;
    match axis {
        'x' => rotate_around_x(vertices, theta),
        'y' => rotate_around_y(vertices, theta),
        'z' => rotate_around_z(vertices, theta),
        _ => panic!("Wrong character passed to rotate_point"),
    }
}

fn rotate_around_x(vertices: &mut Vec<Vec3>, theta: f32) {
    for vertex in vertices {
        let new_y = vertex.y * theta.cos() - vertex.z * theta.sin();
        let new_z = vertex.y * theta.sin() + vertex.z * theta.cos();
        vertex.y = new_y;
        vertex.z = new_z;
    }
}

fn rotate_around_y(vertices: &mut Vec<Vec3>, theta: f32) {
    for vertex in vertices {
        let new_x = vertex.x * theta.cos() + vertex.z * theta.sin();
        let new_z = -vertex.x * theta.sin() + vertex.z * theta.cos();
        vertex.x = new_x;
        vertex.z = new_z;
    }
}

fn rotate_around_z(vertices: &mut Vec<Vec3>, theta: f32) {
    for vertex in vertices {
        let new_x = vertex.x * theta.cos() - vertex.y * theta.sin();
        let new_y = vertex.x * theta.sin() + vertex.y * theta.cos();
        vertex.x = new_x;
        vertex.y = new_y;
    }
}
