use crate::linear_algebra::vectors::{mult_vec, normalize_vec, sub_vec};
use crate::{HEIGHT, WIDTH};
use nalgebra::base::{Matrix4, Vector4};

use super::vectors::{cross_product, dot_product};

pub fn create_projection_matrix() -> Matrix4<f32> {
    let mut proj_matrix: Matrix4<f32> = Matrix4::zeros();

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
    proj_matrix[(2, 3)] = -1.0;
    proj_matrix[(3, 3)] = 0.0;

    proj_matrix
}

pub fn create_x_rot_mat(theta: &f32) -> Matrix4<f32> {
    let mut mat: Matrix4<f32> = Matrix4::zeros();
    let theta = theta.to_radians();

    mat[(0, 0)] = 1.;
    mat[(1, 1)] = (theta * 0.5).cos();
    mat[(1, 2)] = (theta * 0.5).sin();
    mat[(2, 1)] = -(theta * 0.5).sin();
    mat[(2, 2)] = (theta * 0.5).cos();
    mat[(3, 3)] = 1.;

    mat
}

pub fn create_z_rot_mat(theta: &f32) -> Matrix4<f32> {
    let mut mat: Matrix4<f32> = Matrix4::zeros();
    let theta = theta.to_radians();

    mat[(0, 0)] = (theta).cos();
    mat[(0, 1)] = (theta).sin();
    mat[(1, 0)] = -(theta).sin();
    mat[(1, 1)] = (theta).cos();
    mat[(2, 2)] = 1.;
    mat[(3, 3)] = 1.;

    mat
}

pub fn create_y_rot_mat(theta: &f32) -> Matrix4<f32> {
    let mut mat: Matrix4<f32> = Matrix4::zeros();
    let theta = theta.to_radians();

    mat[(0, 0)] = theta.cos();
    mat[(0, 2)] = (theta).sin();
    mat[(2, 0)] = -(theta).sin();
    mat[(1, 1)] = 1.;
    mat[(2, 2)] = (theta).cos();
    mat[(3, 3)] = 1.;

    mat
}
pub fn multiply_matrix_vec(mat: &Matrix4<f32>, input_vec: &Vector4<f32>) -> Vector4<f32> {
    Vector4::new(
        input_vec.x * mat[(0, 0)]
            + input_vec.y * mat[(1, 0)]
            + input_vec.z * mat[(2, 0)]
            + mat[(3, 0)],
        input_vec.x * mat[(0, 1)]
            + input_vec.y * mat[(1, 1)]
            + input_vec.z * mat[(2, 1)]
            + mat[(3, 1)],
        input_vec.x * mat[(0, 2)]
            + input_vec.y * mat[(1, 2)]
            + input_vec.z * mat[(2, 2)]
            + mat[(3, 2)],
        input_vec.x * mat[(0, 3)]
            + input_vec.y * mat[(1, 3)]
            + input_vec.z * mat[(2, 3)]
            + input_vec.w * mat[(3, 3)],
    )
}

pub fn multiply_matrix_vector(input_vec: &Vector4<f32>, mat: &Matrix4<f32>) -> Vector4<f32> {
    let mut projected_vector: Vector4<f32> = Vector4::new(
        input_vec.x * mat[(0, 0)]
            + input_vec.y * mat[(1, 0)]
            + input_vec.z * mat[(2, 0)]
            + mat[(3, 0)],
        input_vec.x * mat[(0, 1)]
            + input_vec.y * mat[(1, 1)]
            + input_vec.z * mat[(2, 1)]
            + mat[(3, 1)],
        input_vec.x * mat[(0, 2)]
            + input_vec.y * mat[(1, 2)]
            + input_vec.z * mat[(2, 2)]
            + mat[(3, 2)],
        1.,
    );

    let w: f32 = input_vec.x * mat[(0, 3)]
        + input_vec.y * mat[(1, 3)]
        + input_vec.z * mat[(2, 3)]
        + mat[(3, 3)];

    if w != 0.0 {
        for i in 0..3 {
            projected_vector[i] /= w
        }
    }

    projected_vector
}

pub fn multiply_matrices(m1: &Matrix4<f32>, m2: &Matrix4<f32>) -> Matrix4<f32> {
    let mut multiplied: Matrix4<f32> = Matrix4::zeros();

    for c in 0..4 {
        for r in 0..4 {
            multiplied[(r, c)] = m1[(r, 0)] * m2[(0, c)]
                + m1[(r, 1)] * m2[(1, c)]
                + m1[(r, 2)] * m2[(2, c)]
                + m1[(r, 3)] * m2[(3, c)];
        }
    }

    multiplied
}

pub fn invert_matrix(mat: &Matrix4<f32>) -> Matrix4<f32> {
    #[rustfmt::skip]
    let mut inverted: Matrix4<f32> = Matrix4::new(mat[(0,0)], mat[(1,0)], mat[(2,0)], 0.,
                                                mat[(0,1)],mat[(1,1)],mat[(2,1)], 0.,
                                                mat[(0,2)],mat[(1,2)],mat[(2,2)], 0.,
                                                0.,0.,0.,   1. );

    inverted[(3, 0)] = -(mat[(3, 0)] * inverted[(0, 0)]
        + mat[(3, 1)] * inverted[(1, 0)]
        + mat[(3, 2)] * inverted[(2, 0)]);
    inverted[(3, 1)] = -(mat[(3, 0)] * inverted[(0, 1)]
        + mat[(3, 1)] * inverted[(1, 1)]
        + mat[(3, 2)] * inverted[(2, 1)]);
    inverted[(3, 2)] = -(mat[(3, 0)] * inverted[(0, 2)]
        + mat[(3, 1)] * inverted[(1, 2)]
        + mat[(3, 2)] * inverted[(2, 2)]);

    inverted
}

pub fn create_point_at_matrix(
    position: Vector4<f32>,
    target: Vector4<f32>,
    up: Vector4<f32>,
) -> Matrix4<f32> {
    let new_forward_dir: Vector4<f32> = normalize_vec(&sub_vec(&target, &position));

    let a: Vector4<f32> = mult_vec(&new_forward_dir, dot_product(&up, &new_forward_dir));
    let new_up_dir: Vector4<f32> = normalize_vec(&sub_vec(&up, &a));

    let new_right_dir: Vector4<f32> = cross_product(&new_up_dir, &new_forward_dir);

    // return point at matrix
    #[rustfmt::skip]
    let point_at: Matrix4<f32> = Matrix4::new(new_right_dir.x, new_right_dir.y, new_right_dir.z, 0.,
                                                    new_up_dir.x, new_up_dir.y, new_up_dir.z, 0.,
                                                    new_forward_dir.x, new_forward_dir.y, new_forward_dir.z, 0.,
                                                    position.x, position.y, position.z, 1.,
                                                );

    point_at
}

pub fn create_trans_matrix(x: f32, y: f32, z: f32) -> Matrix4<f32> {
    let mut matrix: Matrix4<f32> = Matrix4::zeros();

    matrix[(0, 0)] = 1.;
    matrix[(1, 1)] = 1.;
    matrix[(2, 2)] = 1.;
    matrix[(3, 3)] = 1.;
    matrix[(3, 0)] = x;
    matrix[(3, 1)] = y;
    matrix[(3, 2)] = z;

    matrix
}

pub fn world_matrix(
    trans_vec: &Vector4<f32>,
    z_rot_m: &Matrix4<f32>,
    x_rot_m: &Matrix4<f32>,
) -> Matrix4<f32> {
    let world_matrix: Matrix4<f32> = multiply_matrices(&z_rot_m, &x_rot_m);

    multiply_matrices(
        &world_matrix,
        &create_trans_matrix(trans_vec.x, trans_vec.y, trans_vec.z),
    )
}
