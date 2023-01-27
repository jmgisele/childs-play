use super::vectors::{cross_product, get_line, normalize_vec};
use nalgebra::base::Vector4;
use raqote::SolidSource;

#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Vector4<f32>; 3],
    pub color: SolidSource,
}

impl Triangle {
    pub fn get_avg_z(&self) -> f32 {
        (self.vertices[0].z + self.vertices[0].z + self.vertices[0].z) / 3.
    }
}

impl Clone for Triangle {
    fn clone(&self) -> Self {
        Triangle {
            vertices: self.vertices,
            color: self.color,
        }
    }
}
impl Default for Triangle {
    fn default() -> Self {
        Triangle {
            vertices: [
                Vector4::new(0., 0., 0., 1.),
                Vector4::new(0., 0., 0., 1.),
                Vector4::new(0., 0., 0., 1.),
            ],
            color: SolidSource {
                r: 0xd6,
                g: 0x7a,
                b: 0x67,
                a: 0xff,
            },
        }
    }
}

pub fn derive_normal(triangle: &Triangle) -> Vector4<f32> {
    let line_1: Vector4<f32> = get_line(&triangle.vertices[0], &triangle.vertices[1]);

    let line_2: Vector4<f32> = get_line(&triangle.vertices[0], &triangle.vertices[2]);

    normalize_vec(&cross_product(&line_1, &line_2))
}
