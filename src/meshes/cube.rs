use nalgebra::Vector4;

use crate::linear_algebra::data::Mesh;
use crate::linear_algebra::triangles::Triangle;

pub fn _get_cube_mesh() -> Mesh {
    let mut cube: Mesh = Mesh {
        triangles: Vec::new(),
    };

    let triangles: Vec<Triangle> = vec![
        // south
        Triangle {
            vertices: [
                Vector4::new(0., 0., 0., 1.),
                Vector4::new(0., 1., 0., 1.),
                Vector4::new(1., 1., 0., 1.),
            ],
            ..Default::default()
        },
        Triangle {
            vertices: [
                Vector4::new(0., 0., 0., 1.),
                Vector4::new(1., 1., 0., 1.),
                Vector4::new(1., 0., 0., 1.),
            ],
            ..Default::default()
        },
        // east
        Triangle {
            vertices: [
                Vector4::new(1., 0., 0., 1.),
                Vector4::new(1., 1., 0., 1.),
                Vector4::new(1., 1., 1., 1.),
            ],
            ..Default::default()
        },
        Triangle {
            vertices: [
                Vector4::new(1., 0., 0., 1.),
                Vector4::new(1., 1., 1., 1.),
                Vector4::new(1., 0., 1., 1.),
            ],
            ..Default::default()
        },
        // north
        Triangle {
            vertices: [
                Vector4::new(1., 0., 1., 1.),
                Vector4::new(1., 1., 1., 1.),
                Vector4::new(0., 1., 1., 1.),
            ],
            ..Default::default()
        },
        Triangle {
            vertices: [
                Vector4::new(1., 0., 1., 1.),
                Vector4::new(0., 1., 1., 1.),
                Vector4::new(0., 0., 1., 1.),
            ],
            ..Default::default()
        },
        // west
        Triangle {
            vertices: [
                Vector4::new(0., 0., 1., 1.),
                Vector4::new(0., 1., 1., 1.),
                Vector4::new(0., 1., 0., 1.),
            ],
            ..Default::default()
        },
        Triangle {
            vertices: [
                Vector4::new(0., 0., 1., 1.),
                Vector4::new(0., 1., 0., 1.),
                Vector4::new(0., 0., 0., 1.),
            ],
            ..Default::default()
        },
        // top
        Triangle {
            vertices: [
                Vector4::new(0., 1., 0., 1.),
                Vector4::new(0., 1., 1., 1.),
                Vector4::new(1., 1., 1., 1.),
            ],
            ..Default::default()
        },
        Triangle {
            vertices: [
                Vector4::new(0., 1., 0., 1.),
                Vector4::new(1., 1., 1., 1.),
                Vector4::new(1., 1., 0., 1.),
            ],
            ..Default::default()
        },
        // bottom
        Triangle {
            vertices: [
                Vector4::new(1., 0., 1., 1.),
                Vector4::new(0., 0., 1., 1.),
                Vector4::new(0., 0., 0., 1.),
            ],
            ..Default::default()
        },
        Triangle {
            vertices: [
                Vector4::new(1., 0., 1., 1.),
                Vector4::new(0., 0., 0., 1.),
                Vector4::new(1., 0., 0., 1.),
            ],
            ..Default::default()
        },
    ];

    cube.triangles = triangles;

    cube
}
