use crate::linear_algebra::triangles::Triangle;
use nannou::prelude::*;
use raqote::PathBuilder;

pub fn draw_triangle(pb: &mut PathBuilder, triangle: &Triangle) -> () {
    pb.move_to(triangle.vertices[0].x as f32, triangle.vertices[0].y as f32);
    pb.line_to(triangle.vertices[1].x as f32, triangle.vertices[1].y as f32);
    pb.line_to(triangle.vertices[2].x as f32, triangle.vertices[2].y as f32);
    pb.line_to(triangle.vertices[0].x as f32, triangle.vertices[0].y as f32);
}

pub fn draw_triangle_nannou(draw: Draw, triangle: &Triangle) -> () {
    let pt_1 = pt2(triangle.vertices[0].x, triangle.vertices[0].y);
    let pt_2 = pt2(triangle.vertices[1].x, triangle.vertices[1].y);
    let pt_3 = pt2(triangle.vertices[2].x, triangle.vertices[2].y);

    draw.tri()
        .points(pt_1, pt_2, pt_3)
        .color(PINK)
        .stroke_weight(2.0);
}
