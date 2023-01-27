use crate::camera::lightsource::add_lightsource;
use crate::drawing::render::render;
use crate::linear_algebra::data::Mesh;
use crate::linear_algebra::matrices::{
    create_projection_matrix, create_x_rot_mat, create_z_rot_mat,
};
use crate::linear_algebra::triangles::{derive_normal, Triangle};
use crate::linear_algebra::vectors::{add_vec, scale_x_y, sub_vec};
use crate::meshes::initialize_mesh::get_mesh;

use linear_algebra::matrices::{
    create_point_at_matrix, create_y_rot_mat, invert_matrix, multiply_matrix_vec,
    multiply_matrix_vector, world_matrix,
};
use linear_algebra::vectors::{dot_product, get_line, mult_vec};
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

    // setting up mesh
    let mut mesh: Mesh = get_mesh("src/meshes/meshes/axis.obj");

    let projection_matrix: Matrix4<f32> = create_projection_matrix();
    let trans_vec: Vector4<f32> = Vector4::new(0., 0., 6., 0.);
    let mut yaw = 0.;
    let theta: f32 = 0.;
    let speed = 0.1;

    let up: Vector4<f32> = Vector4::new(0., 1., 0., 1.);
    let mut camera: Vector4<f32> = Vector4::new(0., 0., 0., 1.);
    let mut look_dir: Vector4<f32> = Vector4::new(0., 0., 1., 0.);
    let mut target: Vector4<f32> = Vector4::new(0., 0., 1., 1.);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut dt = DrawTarget::new(WIDTH as i32, HEIGHT as i32);

        // clear screen
        dt.clear(SolidSource::from_unpremultiplied_argb(
            0x29, 0x2c, 0x3c, 0xff,
        ));

        // user input
        // back and forth
        if window.is_key_down(Key::Up) {
            camera.y += 2.;
        }
        if window.is_key_down(Key::Left) {
            camera.x += 2.;
        }
        if window.is_key_down(Key::Down) {
            camera.y -= 2.;
        }
        if window.is_key_down(Key::Right) {
            camera.x -= 2.;
        }

        // turning camera
        // up + down
        let forward_vel: Vector4<f32> = mult_vec(&look_dir, speed);
        // if window.is_key_down(Key::W) {
        //     camera = add_vec(&camera, &forward_vel);
        // }

        // if window.is_key_down(Key::S) {
        //     camera = sub_vec(&camera, &forward_vel);
        // }

        // // l + r
        // if window.is_key_down(Key::A) {
        //     yaw -= 0.1;
        // }

        // if window.is_key_down(Key::D) {
        //     yaw += 0.1;
        // }

        // increment rotation angle & rotation matrices
        // theta += 10.;
        let x_rot_matrix: Matrix4<f32> = create_x_rot_mat(&theta);
        let z_rot_matrix: Matrix4<f32> = create_z_rot_mat(&theta);
        let world_matrix: Matrix4<f32> = world_matrix(&trans_vec, &z_rot_matrix, &x_rot_matrix);

        // camera stuff
        let rotated_cam: Matrix4<f32> = create_y_rot_mat(&yaw);
        look_dir = multiply_matrix_vec(&rotated_cam, &target);
        target = add_vec(&camera, &look_dir);
        let point: Matrix4<f32> = create_point_at_matrix(camera, target, up);
        let view: Matrix4<f32> = invert_matrix(&point);

        // get a queue to later order
        let mut triangle_queue: Vec<Triangle> = Vec::new();

        for triangle in mesh.triangles.iter_mut() {
            let mut trans_triangle: Triangle = Triangle { ..*triangle };

            // world matrix
            for i in 0..3 {
                trans_triangle.vertices[i] =
                    multiply_matrix_vector(&trans_triangle.vertices[i], &world_matrix)
            }

            //normals
            let normal: Vector4<f32> = derive_normal(&trans_triangle);

            let camera_ray = sub_vec(&trans_triangle.vertices[0], &camera);

            if dot_product(&normal, &camera_ray) < 0. {
                // world space -> view space
                for i in 0..3 {
                    trans_triangle.vertices[i] =
                        multiply_matrix_vec(&view, &trans_triangle.vertices[i]);
                }

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
