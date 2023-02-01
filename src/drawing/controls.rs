use crate::linear_algebra::vectors::{add_vec, mult_vec, sub_vec};
use minifb::{Key, KeyRepeat, Window};
use nalgebra::base::Vector4;
use raqote::{DrawTarget, SolidSource};

pub fn initialize_user_controls(
    window: &Window,
    dt: &mut DrawTarget,
    camera: &mut Vector4<f32>,
    look_dir: &mut Vector4<f32>,
    yaw: &mut f32,
    speed: &f32,
) {
    // clear screen
    dt.clear(SolidSource::from_unpremultiplied_argb(
        0x29, 0x2c, 0x3c, 0xff,
    ));

    // user input
    // back and forth
    if window.is_key_down(Key::Up) {
        camera.y += 0.2;
    }
    if window.is_key_down(Key::Left) {
        camera.x += 0.2;
    }
    if window.is_key_down(Key::Down) {
        camera.y -= 0.2;
    }
    if window.is_key_down(Key::Right) {
        camera.x -= 0.2;
    }
    // reset
    if window.is_key_down(Key::Tab) {
        *camera = Vector4::new(0., 0., 0., 1.);
        *look_dir = Vector4::new(0., 0., 1., 1.);
        *yaw = 0.;
    }

    // turning camera
    // up + down
    let forward_vel: Vector4<f32> = mult_vec(&look_dir, *speed);
    if window.is_key_down(Key::W) {
        *camera = add_vec(&camera, &forward_vel);
    }

    if window.is_key_down(Key::S) {
        *camera = sub_vec(&camera, &forward_vel);
    }

    // l + r
    if window.is_key_down(Key::A) {
        *yaw -= 1.;
    }

    if window.is_key_down(Key::D) {
        *yaw += 1.;
    }
}
