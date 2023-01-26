use nalgebra::Vector4;

use crate::{Mesh, Triangle};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn get_mesh(file_path: &str) -> Mesh {
    let mut mesh: Mesh = Mesh {
        triangles: Vec::new(),
    };

    let mut vertices_list: Vec<Vector4<f32>> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if &ip[..1] == "v" {
                    let full_line: String = String::from(&ip[2..]);
                    let split_str: Vec<&str> = full_line.split(' ').collect();
                    let mut split_float: Vec<f32> = Vec::new();

                    for i in 0..3 {
                        split_float.push(split_str[i].parse::<f32>().unwrap());
                    }

                    vertices_list.push(Vector4::new(
                        split_float[0],
                        split_float[1],
                        split_float[2],
                        1.,
                    ));
                } else if &ip[..1] == "f" {
                    let full_line: String = String::from(&ip[2..]);
                    let split_str: Vec<&str> = full_line.split(' ').collect();
                    let mut split_int: Vec<usize> = Vec::new();
                    for i in 0..3 {
                        split_int.push(split_str[i].parse::<usize>().unwrap());
                    }
                    let first = split_int[0] - 1;
                    let second = split_int[1] - 1;
                    let third = split_int[2] - 1;

                    let tri: Triangle = Triangle {
                        vertices: [
                            vertices_list[first],
                            vertices_list[second],
                            vertices_list[third],
                        ],
                        ..Default::default()
                    };
                    mesh.triangles.push(tri);
                }
            }
        }
    }
    mesh
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
