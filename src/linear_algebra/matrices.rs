use crate::linear_algebra::vectors::Vec3;
use crate::{HEIGHT, WIDTH};
use nalgebra::base::{Matrix4, Vector3};

pub fn create_projection_matrix() -> Matrix4<f32> {
    let mut proj_matrix: Matrix4<f32> = Matrix4::from_element(0.0);

    let f_near = 0.1;
    let f_far = 1000.0;
    let f_fov = 90.0;
    let f_aspect_ratio = (HEIGHT / WIDTH) as f32;
    let f_calc: f32 = f_fov * 0.5 / 180.0 * 3.14159;
    let f_fov_rad = 1.0 / (f_calc).tan();

    proj_matrix[(0, 0)] = f_aspect_ratio * f_fov_rad;
    proj_matrix[(1, 1)] = f_fov_rad;
    proj_matrix[(2, 2)] = f_far / (f_far - f_near);
    proj_matrix[(3, 2)] = (-f_far * f_near) / (f_far - f_near);
    proj_matrix[(2, 3)] = 1.0;
    proj_matrix[(3, 3)] = 0.0;

    proj_matrix
}

pub fn create_x_rot_mat(theta: &f32) -> Matrix4<f32> {
    let mut mat: Matrix4<f32> = Matrix4::from_element(0.0);
    let theta = theta.to_radians();

    // Rotation X
    mat[(0, 0)] = 1.;
    mat[(1, 1)] = (theta * 0.5).cos();
    mat[(1, 2)] = (theta * 0.5).sin();
    mat[(2, 1)] = -(theta * 0.5).sin();
    mat[(2, 2)] = (theta * 0.5).cos();
    mat[(3, 3)] = 1.;

    mat
}
pub fn create_z_rot_mat(theta: &f32) -> Matrix4<f32> {
    let mut mat: Matrix4<f32> = Matrix4::from_element(0.0);
    let theta = theta.to_radians();

    // Rotation Z
    mat[(0, 0)] = (theta).cos();
    mat[(0, 1)] = (theta).sin();
    mat[(1, 0)] = -(theta).sin();
    mat[(1, 1)] = (theta).cos();
    mat[(2, 2)] = 1.;
    mat[(3, 3)] = 1.;

    mat
}

pub fn multiply_matrices(input_matrix: &Vec3, output_vec: &mut Vec3, proj_mat: &Matrix4<f32>) {
    *output_vec = Vec3 {
        x: input_matrix.x * proj_mat[(0, 0)]
            + input_matrix.y * proj_mat[(1, 0)]
            + input_matrix.z * proj_mat[(2, 0)]
            + proj_mat[(3, 0)],
        y: input_matrix.x * proj_mat[(0, 1)]
            + input_matrix.y * proj_mat[(1, 1)]
            + input_matrix.z * proj_mat[(2, 1)]
            + proj_mat[(3, 1)],
        z: input_matrix.x * proj_mat[(0, 2)]
            + input_matrix.y * proj_mat[(1, 2)]
            + input_matrix.z * proj_mat[(2, 2)]
            + proj_mat[(3, 2)],
    };

    let w: f32 = input_matrix.x * proj_mat[(0, 3)]
        + input_matrix.y * proj_mat[(1, 3)]
        + input_matrix.z * proj_mat[(2, 3)]
        + proj_mat[(3, 3)];

    if w != 0.0 {
        output_vec.divide_by(w)
    }
}

pub fn world_matrix(theta: &f32, trans_vec: Vec3) -> Matrix4<f32> {
    let world_matrix: Matrix4<f32> = create_z_rot_mat(theta) * create_x_rot_mat(theta);

    #[rustfmt::skip]
    let trans_matrix: Matrix4<f32> = Matrix4::new(1., 0., 0., 0.,
                                                    0., 1., 0., 0.,
                                                    0., 0., 1., 0.,
                                                    trans_vec.x, trans_vec.y, trans_vec.z, 1.);

    world_matrix * trans_matrix
}
