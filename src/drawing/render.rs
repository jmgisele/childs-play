use std::vec;

use crate::drawing::shapes::draw_triangle;
use crate::linear_algebra::plane::triangle_clip_plane;
use crate::linear_algebra::triangles::Triangle;
use crate::{HEIGHT, WIDTH};
use minifb::Window;
use nalgebra::Vector4;
use raqote::{
    DrawOptions, DrawTarget, LineCap, LineJoin, PathBuilder, SolidSource, Source, StrokeStyle,
};

pub fn render(window: &mut Window, triangle_queue: Vec<Triangle>, dt: &mut DrawTarget) {
    for triangle in triangle_queue.iter() {
        let mut clipped: [Triangle; 2] = [Triangle::default(), Triangle::default()];
        let mut vec_of_triangles: Vec<Triangle> = Vec::new();

        vec_of_triangles.push(triangle.clone());
        let mut num_new_triangles = 1;

        for p in 0..4 {
            let mut num_tris_to_add = 0;
            while num_new_triangles > 0 {
                let test = vec_of_triangles.pop().unwrap();
                num_new_triangles -= 1;

                num_tris_to_add = match p {
                    0 => triangle_clip_plane(
                        &Vector4::new(0., 0., 0., 1.),
                        &Vector4::new(0., 1., 0., 1.),
                        &test,
                        &mut clipped,
                    ),
                    1 => triangle_clip_plane(
                        &Vector4::new(0., HEIGHT as f32 - 1., 0., 1.),
                        &Vector4::new(0., -1., 0., 1.),
                        &test,
                        &mut clipped,
                    ),
                    2 => triangle_clip_plane(
                        &Vector4::new(0., 0., 0., 1.),
                        &Vector4::new(1., 0., 0., 1.),
                        &test,
                        &mut clipped,
                    ),
                    3 => triangle_clip_plane(
                        &Vector4::new(WIDTH as f32 - 1., 0., 0., 1.),
                        &Vector4::new(-1., 0., 0., 1.),
                        &test,
                        &mut clipped,
                    ),
                    _ => panic!("your rectangle is a polygon"),
                }
            }
            for w in 0..num_tris_to_add {
                vec_of_triangles.push(clipped[w as usize].clone())
            }
            num_new_triangles = vec_of_triangles.len();
        }

        for final_triangle in vec_of_triangles.iter() {
            // Rasterize triangle
            let mut pb = PathBuilder::new();
            println!("{:#?}", final_triangle);

            draw_triangle(&mut pb, &final_triangle);
            println!("got here");
            let path = pb.finish();

            // color in shape
            dt.fill(
                &path,
                &Source::Solid(final_triangle.color),
                &DrawOptions::new(),
            );

            // coloring in wireframe
            dt.stroke(
                &path,
                &Source::Solid(SolidSource {
                    r: 0x18,
                    g: 0x15,
                    b: 0x15,
                    a: 0xff,
                }),
                &StrokeStyle {
                    cap: LineCap::Round,
                    join: LineJoin::Round,
                    width: 1.,
                    miter_limit: 2.,
                    dash_array: vec![1., 1.],
                    dash_offset: 0.,
                },
                &DrawOptions::new(),
            );
        }
    }
    window
        .update_with_buffer(dt.get_data(), WIDTH, HEIGHT)
        .unwrap();
}

// basic color:
// &Source::Solid(SolidSource {
//     r: 0x18,
//     g: 0x15,
//     b: 0x15,
//     a: 0xff,
// }),
