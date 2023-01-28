use nalgebra::Vector4;

use super::{
    triangles::Triangle,
    vectors::{add_vec, dot_product, mult_vec, normalize_vec, sub_vec},
};

pub fn triangle_clip_plane(
    pl_point: &Vector4<f32>,
    pl_normal: &Vector4<f32>,
    in_tri: &Triangle,
    tri_arr: &mut [Triangle; 2],
) -> i32 {
    let [out_tri_1, out_tri_2] = tri_arr;
    let normal_plane: Vector4<f32> = normalize_vec(pl_normal);

    let distances: [f32; 3] = [
        shortest_dist_pt_plane(&in_tri.vertices[0], &normal_plane, pl_point),
        shortest_dist_pt_plane(&in_tri.vertices[1], &normal_plane, pl_point),
        shortest_dist_pt_plane(&in_tri.vertices[2], &normal_plane, pl_point),
    ];

    let mut outside_points: Vec<Vector4<f32>> = Vec::new();
    let mut outside_count: usize = 0;
    let mut inside_points: Vec<Vector4<f32>> = Vec::new();
    let mut inside_count: usize = 0;

    // if distance is greater than 0, it's inside; if less, it's outside
    for (i, element) in distances.iter().enumerate() {
        if *element >= 0. {
            inside_points.push(in_tri.vertices[i]);
            inside_count += 1;
        } else {
            outside_points.push(in_tri.vertices[i]);
            outside_count += 1;
        }
    }

    if inside_count == 0 {
        // All points lie on the outside of plane, so clip whole triangle
        // No returned triangles are valid
        return 0;
    } else if inside_count == 3 {
        // All points lie on the inside of plane, so do nothing
        // and allow the triangle to simply pass through
        *out_tri_1 = in_tri.clone();

        return 1;
    } else if inside_count == 1 && outside_count == 2 {
        // Triangle should be clipped. As two points lie outside
        // the plane, the triangle simply becomes a smaller triangle
        *out_tri_1 = in_tri.clone();

        // The inside point is valid, so keep that...
        *out_tri_1.vertices[0] = *inside_points[0];

        // but the two new points are at the locations where the
        // original sides of the triangle (lines) intersect with the plane
        *out_tri_1.vertices[1] =
            *vec_intersect_plane(&pl_point, &pl_normal, &inside_points[0], &outside_points[0]);
        *out_tri_1.vertices[2] =
            *vec_intersect_plane(&pl_point, &pl_normal, &inside_points[0], &outside_points[1]);

        return 1; // Return the newly formed single triangle
    } else {
        *out_tri_1 = in_tri.clone();
        *out_tri_2 = in_tri.clone();

        // The first triangle consists of the two inside points and a new
        // point determined by the location where one side of the triangle
        // intersects with the plane
        *out_tri_1.vertices[0] = *inside_points[0];
        *out_tri_1.vertices[1] = *inside_points[1];
        *out_tri_1.vertices[2] =
            *vec_intersect_plane(&pl_point, &pl_normal, &inside_points[0], &outside_points[0]);

        // The second triangle is composed of one of he inside points, a
        // new point determined by the intersection of the other side of the
        // triangle and the plane, and the newly created point above
        *out_tri_2.vertices[0] = *inside_points[1];
        *out_tri_2.vertices[1] = *out_tri_1.vertices[2];
        *out_tri_2.vertices[2] =
            *vec_intersect_plane(&pl_point, &pl_normal, &inside_points[1], &outside_points[0]);
        return 2;
    }
}

pub fn shortest_dist_pt_plane(
    point: &Vector4<f32>,
    plane_normal: &Vector4<f32>,
    plane_point: &Vector4<f32>,
) -> f32 {
    plane_normal.x * point.x + plane_normal.y * point.y + plane_normal.z * point.z
        - dot_product(&plane_normal, &plane_point)
}

pub fn vec_intersect_plane(
    pl_point: &Vector4<f32>,
    pl_normal: &Vector4<f32>,
    line_s: &Vector4<f32>,
    line_e: &Vector4<f32>,
) -> Vector4<f32> {
    let normal_plane: Vector4<f32> = normalize_vec(pl_normal);

    let plane_dot: f32 = dot_product(&normal_plane, &pl_point);
    let ad = dot_product(&line_s, &normal_plane);
    let bd = dot_product(&line_e, &normal_plane);
    let t: f32 = (plane_dot - ad) / (bd - ad);
    let full_line: Vector4<f32> = sub_vec(&line_e, &line_s);
    let line_intersect = mult_vec(&full_line, t);
    return add_vec(&line_s, &line_intersect);
}
