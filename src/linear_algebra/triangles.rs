use super::matrices::multiply_matrices;
use super::vectors::{cross_product, get_line, Vec3};
use nalgebra::base::Matrix4;
use raqote::SolidSource;

#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Vec3; 3],
    pub color: SolidSource,
}

impl Triangle {
    pub fn get_avg_z(&self) -> f32 {
        (self.vertices[0].z + self.vertices[0].z + self.vertices[0].z) / 3.
    }
}

impl Default for Triangle {
    fn default() -> Triangle {
        Triangle {
            vertices: [
                Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
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

pub fn derive_normal(triangle: &Triangle) -> Vec3 {
    let line_1: Vec3 = get_line(&triangle.vertices[0], &triangle.vertices[1]);

    let line_2: Vec3 = get_line(&triangle.vertices[0], &triangle.vertices[2]);

    (&cross_product(&line_1, &line_2)).normalize_vec()
}

pub fn rotate_triangle(in_m: &Triangle, out_m: &mut Triangle, rot_m: &Matrix4<f32>) {
    for i in 0..3 {
        multiply_matrices(&in_m.vertices[i], &mut out_m.vertices[i], &rot_m);
    }
}

pub fn offset_triangle(tri: &mut Triangle, val: f32) {
    for i in 0..3 {
        tri.vertices[i].add_z(val)
    }
}

pub fn project_3_2_d_tri(in_mat: &Triangle, out_mat: &mut Triangle, proj_mat: &Matrix4<f32>) {
    for i in 0..3 {
        multiply_matrices(&in_mat.vertices[i], &mut out_mat.vertices[i], &proj_mat);
    }
}
