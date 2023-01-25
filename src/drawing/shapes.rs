use crate::linear_algebra::triangles::Triangle;
use raqote::PathBuilder;

pub fn draw_triangle(pb: &mut PathBuilder, triangle: &Triangle) -> () {
    pb.move_to(triangle.vertices[0].x as f32, triangle.vertices[0].y as f32);
    pb.line_to(triangle.vertices[1].x as f32, triangle.vertices[1].y as f32);
    pb.line_to(triangle.vertices[2].x as f32, triangle.vertices[2].y as f32);
    pb.line_to(triangle.vertices[0].x as f32, triangle.vertices[0].y as f32);
}
