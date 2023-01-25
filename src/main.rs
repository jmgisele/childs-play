use crate::camera::lightsource::add_lightsource;
use crate::drawing::render::render;
use crate::linear_algebra::data::{initialize_empty_triangle, Mesh};
use crate::linear_algebra::matrices::{
    create_projection_matrix, create_x_rot_mat, create_z_rot_mat, world_matrix,
};
use crate::linear_algebra::triangles::{
    derive_normal, offset_triangle, project_3_2_d_tri, rotate_triangle, Triangle,
};
use crate::linear_algebra::vectors::{scale_x_y, Vec3};
use crate::meshes::initialize_mesh::get_mesh;

use minifb::{Key, Window, WindowOptions};
use nalgebra::base::Matrix4;
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
    let camera: Vec3 = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut once = true;

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

        // get a queue to later order
        let mut triangle_queue: Vec<Triangle> = Vec::new();

        for triangle in mesh.triangles.iter_mut() {
            // OLD STRAT
            let mut after_z: Triangle = initialize_empty_triangle();
            let mut after_xz: Triangle = initialize_empty_triangle();
            let mut final_triangle: Triangle = initialize_empty_triangle();
            // rotate in z
            rotate_triangle(&triangle, &mut after_z, &z_rot_matrix);
            // rotate in x
            rotate_triangle(&after_z, &mut after_xz, &x_rot_matrix);
            //  Offset
            offset_triangle(&mut after_xz, 6.);
            //normals
            let normal = derive_normal(&after_xz);
            let surface_dot: f32 = normal.x * (after_xz.vertices[0].x - camera.x)
                + normal.y * (after_xz.vertices[0].y - camera.y)
                + normal.z * (after_xz.vertices[0].z - camera.z);

            if surface_dot < 0. {
                // project to 2D
                project_3_2_d_tri(&after_xz, &mut final_triangle, &projection_matrix);

                // Scale into view
                scale_x_y(&mut final_triangle);

                // add light
                add_lightsource(normal, &mut final_triangle);

                // debug
                if once {
                    once = false;
                    println!("final triangle {:#?}", &final_triangle);
                    println!("after_z triangle {:#?}", &after_z);
                    println!("after_xz triangle {:#?}", &after_xz);
                    println!("normal {:#?}", &normal);
                }
                // add to final triangle queue
                triangle_queue.push(final_triangle);
            }
        }

        // order them by z vals
        triangle_queue.sort_by(|a, b| (b.get_avg_z()).partial_cmp(&a.get_avg_z()).unwrap());
        // render
        render(&mut window, triangle_queue, &mut dt)
    }
}
