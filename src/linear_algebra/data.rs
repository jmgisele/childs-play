use crate::linear_algebra::triangles::Triangle;
use crate::linear_algebra::vectors::Vec3;

#[derive(Debug)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

pub fn initialize_empty_triangle() -> Triangle {
    return Triangle {
        vertices: [
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        ],
        ..Default::default()
    };
}
