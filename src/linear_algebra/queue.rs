use crate::camera::lightsource::add_lightsource;
use crate::linear_algebra::data::Mesh;
use crate::linear_algebra::matrices::{
    create_point_at_matrix, create_y_rot_mat, invert_matrix, multiply_matrix_vec, world_matrix,
};
use crate::linear_algebra::matrices::{create_x_rot_mat, create_z_rot_mat};
use crate::linear_algebra::plane::triangle_clip_plane;
use crate::linear_algebra::triangles::{derive_normal, Triangle};
use crate::linear_algebra::vectors::dot_product;
use crate::linear_algebra::vectors::{add_vec, scale_x_y, sub_vec};
use nalgebra::base::{Matrix4, Vector4};

pub fn get_triangle_queue(
    theta: &f32,
    trans_vec: &Vector4<f32>,
    yaw: &f32,
    look_dir: &mut Vector4<f32>,
    camera: &Vector4<f32>,
    mesh: &mut Mesh,
    projection_matrix: Matrix4<f32>,
) -> Vec<Triangle> {
    // increment rotation angle & rotation matrices
    let x_rot_matrix: Matrix4<f32> = create_x_rot_mat(theta);
    let z_rot_matrix: Matrix4<f32> = create_z_rot_mat(theta);
    let world_matrix: Matrix4<f32> = world_matrix(&trans_vec, &z_rot_matrix, &x_rot_matrix);

    // camera stuff
    let up: Vector4<f32> = Vector4::new(0., 1., 0., 1.);
    let mut target: Vector4<f32> = Vector4::new(0., 0., 1., 1.);

    let rotated_cam: Matrix4<f32> = create_y_rot_mat(&yaw);
    *look_dir = multiply_matrix_vec(&rotated_cam, &target);
    target = add_vec(&camera, &look_dir);
    let point: Matrix4<f32> = create_point_at_matrix(*camera, target, up);
    let view: Matrix4<f32> = invert_matrix(&point);

    // get a queue to later order
    let mut triangle_queue: Vec<Triangle> = Vec::new();

    for triangle in mesh.triangles.iter_mut() {
        let mut trans_triangle: Triangle = Triangle { ..*triangle };

        // world matrix
        for i in 0..3 {
            trans_triangle.vertices[i] =
                multiply_matrix_vec(&world_matrix, &trans_triangle.vertices[i])
        }

        //normals
        let normal: Vector4<f32> = derive_normal(&trans_triangle);

        let camera_ray: Vector4<f32> = sub_vec(&trans_triangle.vertices[0], &camera);

        if dot_product(&normal, &camera_ray) < 0. {
            // add light
            add_lightsource(normal, &mut trans_triangle);

            // world space -> view space
            for i in 0..3 {
                trans_triangle.vertices[i] =
                    multiply_matrix_vec(&view, &trans_triangle.vertices[i]);
            }

            // Clip Viewed Triangle against near plane, this could form two additional
            // additional triangles.
            let mut clipped: [Triangle; 2] = [Triangle::default(), Triangle::default()];
            let num_clipped_triangles: i32 = triangle_clip_plane(
                &Vector4::new(0., 0., 1., 1.),
                &Vector4::new(0., 0., 1., 1.),
                &trans_triangle,
                &mut clipped,
            );

            for n in 0..num_clipped_triangles {
                let mut clip_tri: Triangle = clipped[n as usize].clone();
                // project to 2D
                for i in 0..3 {
                    clip_tri.vertices[i] =
                        multiply_matrix_vec(&projection_matrix, &(clip_tri.vertices[i]))
                }

                // Scale into view
                scale_x_y(&mut clip_tri);

                // add to final triangle queue
                triangle_queue.push(clip_tri);
            }
        }
    }

    // order them by z vals
    triangle_queue.sort_by(|a, b| (b.get_avg_z()).partial_cmp(&a.get_avg_z()).unwrap());

    triangle_queue
}
