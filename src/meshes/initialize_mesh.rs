use crate::{Mesh, Triangle, Vec3};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn _create_initial_triangles() -> Mesh {
    return Mesh {
        triangles: vec![
            // south
            Triangle {
                vertices: [
                    Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    Vec3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                ],
                ..Default::default()
            },
            Triangle {
                vertices: [
                    Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    Vec3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                ],
                ..Default::default()
            },
            // east
            Triangle {
                vertices: [
                    Vec3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                ],
                ..Default::default()
            },
            Triangle {
                vertices: [
                    Vec3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    Vec3 {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                ],
                ..Default::default()
            },
            // north
            Triangle {
                vertices: [
                    Vec3 {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    Vec3 {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                    },
                ],
                ..Default::default()
            },
            Triangle {
                vertices: [
                    Vec3 {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    Vec3 {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                ],
                ..Default::default()
            },
            // west
            Triangle {
                vertices: [
                    Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    Vec3 {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    Vec3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                ],
                ..Default::default()
            },
            Triangle {
                vertices: [
                    Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    Vec3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                ],
                ..Default::default()
            },
            // top
            Triangle {
                vertices: [
                    Vec3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    Vec3 {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                ],
                ..Default::default()
            },
            Triangle {
                vertices: [
                    Vec3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                ],
                ..Default::default()
            },
            // bottom
            Triangle {
                vertices: [
                    Vec3 {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                ],
                ..Default::default()
            },
            Triangle {
                vertices: [
                    Vec3 {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    Vec3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                ],
                ..Default::default()
            },
        ],
    };
}

pub fn get_mesh(file_path: &str) -> Mesh {
    let mut mesh: Mesh = Mesh {
        triangles: Vec::new(),
    };

    let mut vertices_list: Vec<Vec3> = Vec::new();

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

                    vertices_list.push(Vec3 {
                        x: split_float[0],
                        y: split_float[1],
                        z: split_float[2],
                    });
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
