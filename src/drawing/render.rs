use crate::drawing::shapes::draw_triangle;
use crate::linear_algebra::triangles::Triangle;
use crate::{HEIGHT, WIDTH};
use minifb::Window;
use raqote::{DrawOptions, DrawTarget, LineCap, LineJoin, PathBuilder, Source, StrokeStyle};

pub fn render(window: &mut Window, triangle_queue: Vec<Triangle>, dt: &mut DrawTarget) {
    for final_triangle in triangle_queue.iter() {
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
            &Source::Solid(final_triangle.color),
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
