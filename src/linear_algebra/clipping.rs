use nalgebra::Vector4;

use crate::{HEIGHT, WIDTH};

use super::{plane::triangle_clip_plane, triangles::Triangle};

pub fn clip_against_window(
    clipped: &mut [Triangle; 2],
    vec_of_triangles: &mut Vec<Triangle>,
    triangle: &Triangle,
) {
    vec_of_triangles.push(triangle.clone());
    let mut num_new_triangles = 1;

    for p in 0..4 {
        while num_new_triangles > 0 {
            let test: Triangle = vec_of_triangles.pop().expect("expected a triangle");
            num_new_triangles -= 1;

            let num_tris_to_add = match p {
                0 => triangle_clip_plane(
                    &Vector4::new(0., 0., 0., 1.),
                    &Vector4::new(0., 1., 0., 1.),
                    &test,
                    clipped,
                ),
                1 => triangle_clip_plane(
                    &Vector4::new(0., HEIGHT as f32 - 1., 0., 1.),
                    &Vector4::new(0., -1., 0., 1.),
                    &test,
                    clipped,
                ),
                2 => triangle_clip_plane(
                    &Vector4::new(0., 0., 0., 1.),
                    &Vector4::new(1., 0., 0., 1.),
                    &test,
                    clipped,
                ),
                3 => triangle_clip_plane(
                    &Vector4::new(WIDTH as f32 - 1., 0., 0., 1.),
                    &Vector4::new(-1., 0., 0., 1.),
                    &test,
                    clipped,
                ),
                _ => panic!("your rectangle is a polygon"),
            };
            for w in 0..num_tris_to_add {
                vec_of_triangles.push(clipped[w as usize].clone())
            }
        }

        num_new_triangles = vec_of_triangles.len();
    }
}
