use crate::structs::Vec3;
use std::fs;

pub fn read_file(file_name: &str) -> Result<Vec<String>, std::io::Error> {
    let data = fs::read_to_string(file_name)?;
    Ok(data.lines().map(|l| l.to_string()).collect())
}

pub fn parse_lines(lines: Vec<String>) -> (Vec<Vec3>, Vec<(usize, usize)>) {
    let mut vertices = Vec::new();
    let mut edges = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.as_slice() {
            ["v", x, y, z] => vertices.push(Vec3 {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                z: z.parse().unwrap(),
            }),
            ["e", a, b] => edges.push((a.parse().unwrap(), b.parse().unwrap())),
            _ => {}
        }
    }
    (vertices, edges)
}
