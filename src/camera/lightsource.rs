use crate::drawing::colors::get_color;
use crate::linear_algebra::triangles::Triangle;
use crate::linear_algebra::vectors::Vec3;
pub fn add_lightsource(normal: Vec3, triangle: &mut Triangle) {
    let mut light_dir: Vec3 = Vec3 {
        x: 0.,
        y: 0.,
        z: -1.,
    };
    let light_len: f32 =
        (light_dir.x * light_dir.x + light_dir.y * light_dir.y + light_dir.z * light_dir.z).sqrt();

    light_dir.x /= light_len;
    light_dir.y /= light_len;
    light_dir.z /= light_len;

    let light_dot = normal.x * light_dir.x + normal.y * light_dir.y + normal.z * light_dir.z;
    triangle.color = get_color(light_dot);
}
