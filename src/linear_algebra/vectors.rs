use crate::{Triangle, HEIGHT};

pub fn scale_x_y(triangle: &mut Triangle) {
    for i in 0..3 {
        triangle.vertices[i].x = (triangle.vertices[i].x + 1.) * 0.5 * HEIGHT as f32;
        triangle.vertices[i].y = (triangle.vertices[i].y + 1.) * 0.5 * HEIGHT as f32;
    }
}

pub fn get_line(vec1: &Vec3, vec2: &Vec3) -> Vec3 {
    Vec3 {
        x: vec2.x - vec1.x,
        y: vec2.y - vec1.y,
        z: vec2.z - vec1.z,
    }
}

pub fn dot_product(vec1: &Vec3, vec2: &Vec3) -> f32 {
    vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z
}

pub fn cross_product(vec1: &Vec3, vec2: &Vec3) -> Vec3 {
    Vec3 {
        x: vec1.y * vec2.z - vec1.z * vec2.y,
        y: vec1.z * vec2.x - vec1.x * vec2.z,
        z: vec1.x * vec2.y - vec1.y * vec2.x,
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}
impl Vec3 {
    pub fn divide_by(&mut self, val: f32) {
        self.x /= val;
        self.y /= val;
        self.z /= val;
    }

    pub fn add_z(&mut self, val: f32) {
        self.z += val;
    }

    pub fn vec_magnitude(&self) -> f32 {
        dot_product(&self, &self).sqrt()
    }

    pub fn normalize_vec(&self) -> Vec3 {
        let mag = self.vec_magnitude();

        Vec3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
}
