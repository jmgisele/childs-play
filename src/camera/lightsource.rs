use nalgebra::Vector4;

use crate::drawing::colors::get_color;
use crate::linear_algebra::triangles::Triangle;
use crate::linear_algebra::vectors::{dot_product, normalize_vec};

pub fn add_lightsource(normal: Vector4<f32>, triangle: &mut Triangle) {
    let mut light_dir: Vector4<f32> = Vector4::new(0., 1., -1., 1.);

    light_dir = normalize_vec(&light_dir);

    let light_dot = dot_product(&normal, &light_dir);

    triangle.color = get_color(light_dot);
}
