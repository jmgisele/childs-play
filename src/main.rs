use crate::camera::lightsource::add_lightsource;
use crate::drawing::render::render;
use crate::linear_algebra::data::Mesh;
use crate::linear_algebra::matrices::{
    create_projection_matrix, create_x_rot_mat, create_z_rot_mat,
};
use crate::linear_algebra::triangles::{derive_normal, Triangle};
use crate::linear_algebra::vectors::scale_x_y;
use crate::meshes::initialize_mesh::get_mesh;

use linear_algebra::matrices::{multiply_matrix_vector, world_matrix};
use linear_algebra::vectors::{dot_product, get_line};
use minifb::{Key, Window, WindowOptions};
use nalgebra::base::{Matrix4, Vector4};
use raqote::{DrawTarget, SolidSource};

mod camera;
mod drawing;
mod linear_algebra;
mod meshes;

pub const WIDTH: usize = 400;
pub const HEIGHT: usize = 400;

fn main() {
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

    // setting up basics
    // let mut mesh: Mesh = create_initial_triangles();
    let mut mesh: Mesh = get_mesh("src/meshes/meshes/video_ship.obj");
    let projection_matrix: Matrix4<f32> = create_projection_matrix();
    let mut theta = 1.;
    let camera: Vector4<f32> = Vector4::new(0., 0., 0., 0.);
    let trans_vec: Vector4<f32> = Vector4::new(0., 0., 6., 0.);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut dt = DrawTarget::new(WIDTH as i32, HEIGHT as i32);

        // clear screen
        dt.clear(SolidSource::from_unpremultiplied_argb(
            0x29, 0x2c, 0x3c, 0xff,
        ));

        // increment rotation angle & rotation matrices
        theta += 10.;
        let x_rot_matrix: Matrix4<f32> = create_x_rot_mat(&theta);
        let z_rot_matrix: Matrix4<f32> = create_z_rot_mat(&theta);
        let world_matrix: Matrix4<f32> = world_matrix(&trans_vec, &z_rot_matrix, &x_rot_matrix);

        // get a queue to later order
        let mut triangle_queue: Vec<Triangle> = Vec::new();

        for triangle in mesh.triangles.iter_mut() {
            let mut trans_triangle: Triangle = Triangle { ..*triangle };

            for i in 0..3 {
                trans_triangle.vertices[i] =
                    multiply_matrix_vector(&trans_triangle.vertices[i], &world_matrix)
            }

            //normals
            let normal: Vector4<f32> = derive_normal(&trans_triangle);

            let surface_dot: f32 =
                dot_product(&normal, &get_line(&camera, &trans_triangle.vertices[0]));

            if surface_dot < 0. {
                // project to 2D
                for i in 0..3 {
                    trans_triangle.vertices[i] =
                        multiply_matrix_vector(&trans_triangle.vertices[i], &projection_matrix)
                }

                // Scale into view
                scale_x_y(&mut trans_triangle);

                // add light
                add_lightsource(normal, &mut trans_triangle);

                // add to final triangle queue
                triangle_queue.push(trans_triangle);
            }
        }

        // order them by z vals
        triangle_queue.sort_by(|a, b| (b.get_avg_z()).partial_cmp(&a.get_avg_z()).unwrap());
        // render
        render(&mut window, triangle_queue, &mut dt)
    }
}
