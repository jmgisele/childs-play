use crate::linear_algebra::triangles::Triangle;
use crate::linear_algebra::vectors::Vec3;

#[derive(Debug)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}
