use crate::{Triangle, HEIGHT, WIDTH};
use nalgebra::base::Vector4;

pub fn scale_x_y(triangle: &mut Triangle) {
    let offset_view: Vector4<f32> = Vector4::new(1., 1., 0., 0.);

    // scale into cartesian
    for i in 0..3 {
        triangle.vertices[i] = div_vec(&triangle.vertices[i], triangle.vertices[i].w)
    }

    // invert x and y
    // for i in 0..3 {
    //     triangle.vertices[i].x *= -1.;
    //     triangle.vertices[i].y *= -1.;
    // }
    // offset into normal space and invert x and y
    for i in 0..3 {
        triangle.vertices[i] = add_vec(&triangle.vertices[i], &offset_view);
        triangle.vertices[i].x *= (triangle.vertices[i].y + 1.) * 0.5 * WIDTH as f32;
        triangle.vertices[i].y *= (triangle.vertices[i].y + 1.) * 0.5 * HEIGHT as f32;
    }
}

pub fn get_line(vec1: &Vector4<f32>, vec2: &Vector4<f32>) -> Vector4<f32> {
    Vector4::new(vec2.x - vec1.x, vec2.y - vec1.y, vec2.z - vec1.z, 1.)
}

pub fn dot_product(vec1: &Vector4<f32>, vec2: &Vector4<f32>) -> f32 {
    vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z
}

pub fn cross_product(vec1: &Vector4<f32>, vec2: &Vector4<f32>) -> Vector4<f32> {
    Vector4::new(
        vec1.y * vec2.z - vec1.z * vec2.y,
        vec1.z * vec2.x - vec1.x * vec2.z,
        vec1.x * vec2.y - vec1.y * vec2.x,
        1.,
    )
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

pub fn sub_vec(vec1: &Vector4<f32>, vec2: &Vector4<f32>) -> Vector4<f32> {
    Vector4::new(vec1.x - vec2.x, vec1.y - vec2.y, vec1.z - vec2.z, 1.)
}

pub fn add_vec(vec1: &Vector4<f32>, vec2: &Vector4<f32>) -> Vector4<f32> {
    Vector4::new(vec1.x + vec2.x, vec1.y + vec2.y, vec1.z + vec2.z, 1.)
}

pub fn mult_vec(v1: &Vector4<f32>, k: f32) -> Vector4<f32> {
    return Vector4::new(v1.x * k, v1.y * k, v1.z * k, 1.);
}

pub fn div_vec(v1: &Vector4<f32>, k: f32) -> Vector4<f32> {
    return Vector4::new(v1.x / k, v1.y / k, v1.z / k, 1.);
}

pub fn normalize_vec(vec: &Vector4<f32>) -> Vector4<f32> {
    let mag = vec_magnitude(&vec);

    Vector4::new(vec.x / mag, vec.y / mag, vec.z / mag, 1.)
}

pub fn vec_magnitude(vec: &Vector4<f32>) -> f32 {
    dot_product(&vec, &vec).sqrt()
}
