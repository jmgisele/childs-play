use std::vec;

use crate::drawing::shapes::draw_triangle;
use crate::linear_algebra::clipping::clip_against_window;
use crate::linear_algebra::triangles::Triangle;
use crate::{HEIGHT, WIDTH};
use minifb::Window;
use raqote::{
    DrawOptions, DrawTarget, LineCap, LineJoin, PathBuilder, SolidSource, Source, StrokeStyle,
};

pub fn render(window: &mut Window, triangle_queue: Vec<Triangle>, dt: &mut DrawTarget) {
    for triangle in triangle_queue.iter() {
        let mut clipped: [Triangle; 2] = [Triangle::default(), Triangle::default()];
        let mut vec_of_triangles: Vec<Triangle> = Vec::new();

        clip_against_window(&mut clipped, &mut vec_of_triangles, triangle);

        for final_triangle in vec_of_triangles.iter() {
            render_triangle(final_triangle, dt);
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

pub fn render_triangle(triangle: &Triangle, dt: &mut DrawTarget) {
    // Rasterize triangle
    let mut pb = PathBuilder::new();

    draw_triangle(&mut pb, &triangle);
    let path = pb.finish();

    // color in shape
    dt.fill(&path, &Source::Solid(triangle.color), &DrawOptions::new());

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
