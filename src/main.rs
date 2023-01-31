use crate::drawing::render::render;
use crate::linear_algebra::data::Mesh;
use crate::linear_algebra::matrices::create_projection_matrix;
use crate::linear_algebra::triangles::Triangle;
use crate::meshes::initialize_mesh::get_mesh;

use drawing::controls::initialize_user_controls;
use linear_algebra::queue::get_triangle_queue;
use minifb::{Key, Window, WindowOptions};
use nalgebra::base::{Matrix4, Vector4};
use raqote::DrawTarget;

mod camera;
mod drawing;
mod linear_algebra;
mod meshes;

pub const WIDTH: usize = 400;
pub const HEIGHT: usize = 400;

// use nannou::prelude::*;
fn main() {
    // nannou
    // nannou::app(model).event(event).run();

    // raqote
    let mut window = Window::new(
        "Baby Graphics Engine - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60.0 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16660)));

    // setting up mesh
    let mut mesh: Mesh = get_mesh("src/meshes/meshes/video_ship.obj");

    let projection_matrix: Matrix4<f32> = create_projection_matrix();
    let trans_vec: Vector4<f32> = Vector4::new(0., 0., 5., 1.);
    let mut yaw = 0.;
    let theta: f32 = 0.;
    let speed = 0.1;
    let mut camera: Vector4<f32> = Vector4::new(0., 0., 0., 1.);
    let mut look_dir = Vector4::new(0., 0., 1., 1.);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut dt = DrawTarget::new(WIDTH as i32, HEIGHT as i32);

        initialize_user_controls(
            &window,
            &mut dt,
            &mut camera,
            &mut look_dir,
            &mut yaw,
            &speed,
        );

        let triangle_queue = get_triangle_queue(
            &theta,
            &trans_vec,
            &yaw,
            &mut look_dir,
            &camera,
            &mut mesh,
            projection_matrix,
        );
        // render
        render(&mut window, triangle_queue, &mut dt)
    }
}

// struct Model {}

// fn model(app: &App) -> Model {
//     app.new_window().size(512, 512).view(view).build().unwrap();
//     Model {}
// }

// fn view(app: &App, _model: &Model, frame: Frame) {
//     frame.clear(BLACK);
//     let draw = app.draw();

//     draw.to_frame(app, &frame).unwrap();
// }

// fn event(_app: &App, _model: &mut Model, _event: Event) {}
