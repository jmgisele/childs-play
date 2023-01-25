use minifb::{Key, Window, WindowOptions};
use nalgebra::base::Matrix4;
use raqote::{
    DrawOptions, DrawTarget, LineCap, LineJoin, PathBuilder, SolidSource, Source, StrokeStyle,
};

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

fn main() {
    let mut window = Window::new(
        "Test - ESC to exit",
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
    let mut mesh: Mesh = create_initial_triangles();
    let projection_matrix: Matrix4<f32> = create_projection_matrix();
    let mut theta = 1.;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut dt = DrawTarget::new(WIDTH as i32, HEIGHT as i32);

        dt.clear(SolidSource::from_unpremultiplied_argb(
            0xff, 0xff, 0xff, 0xff,
        ));

        theta += 1.;

        let x_rot_matrix = create_x_rot_mat(&theta);
        let z_rot_matrix = create_z_rot_mat(&theta);

        for triangle in mesh.triangles.iter_mut() {
            let initial_triangle: Triangle = initialize_empty_triangle();
            let mut after_z: Triangle = initialize_empty_triangle();
            let mut after_xz: Triangle = initialize_empty_triangle();
            let mut offset_triangle: Triangle = initialize_empty_triangle();

            // rotate in z
            multiply_matrices(&triangle.vertice_1, &mut after_z.vertice_1, &z_rot_matrix);
            multiply_matrices(&triangle.vertice_2, &mut after_z.vertice_2, &z_rot_matrix);
            multiply_matrices(&triangle.vertice_3, &mut after_z.vertice_3, &z_rot_matrix);

            // rotate in x
            multiply_matrices(&after_z.vertice_1, &mut after_xz.vertice_1, &x_rot_matrix);
            multiply_matrices(&after_z.vertice_2, &mut after_xz.vertice_2, &x_rot_matrix);
            multiply_matrices(&after_z.vertice_3, &mut after_xz.vertice_3, &x_rot_matrix);

            //  Offset
            after_xz.vertice_1.add_z(3.);
            after_xz.vertice_2.add_z(3.);
            after_xz.vertice_3.add_z(3.);

            // project to 2D
            multiply_matrices(
                &after_xz.vertice_1,
                &mut offset_triangle.vertice_1,
                &projection_matrix,
            );
            multiply_matrices(
                &after_xz.vertice_2,
                &mut offset_triangle.vertice_2,
                &projection_matrix,
            );
            multiply_matrices(
                &after_xz.vertice_3,
                &mut offset_triangle.vertice_3,
                &projection_matrix,
            );

            // Scale into view
            scale(&mut offset_triangle);

            // Rasterize triangle
            let mut pb = PathBuilder::new();
            draw_triangle(&mut pb, &offset_triangle);
            let path = pb.finish();
            dt.stroke(
                &path,
                &Source::Solid(SolidSource {
                    r: 0x0,
                    g: 0x0,
                    b: 0x80,
                    a: 0x80,
                }),
                &StrokeStyle {
                    cap: LineCap::Round,
                    join: LineJoin::Round,
                    width: 5.,
                    miter_limit: 2.,
                    dash_array: vec![1., 1.],
                    dash_offset: 0.,
                },
                &DrawOptions::new(),
            );
        }
        window
            .update_with_buffer(dt.get_data(), WIDTH, HEIGHT)
            .unwrap();
    }
}

fn draw_triangle(pb: &mut PathBuilder, triangle: &Triangle) -> () {
    pb.move_to(triangle.vertice_1.x as f32, triangle.vertice_1.y as f32);
    pb.line_to(triangle.vertice_2.x as f32, triangle.vertice_2.y as f32);
    pb.line_to(triangle.vertice_3.x as f32, triangle.vertice_3.y as f32);
    pb.line_to(triangle.vertice_1.x as f32, triangle.vertice_1.y as f32);
}

fn scale(triangle: &mut Triangle) {
    // yes i need to figure out how to iterate here lol
    triangle.vertice_1.x += 1.;
    triangle.vertice_1.y += 1.;
    triangle.vertice_2.x += 1.;
    triangle.vertice_2.y += 1.;
    triangle.vertice_3.x += 1.;
    triangle.vertice_3.y += 1.;

    triangle.vertice_1.x *= 0.5 * HEIGHT as f32;
    triangle.vertice_1.y *= 0.5 * HEIGHT as f32;
    triangle.vertice_2.x *= 0.5 * HEIGHT as f32;
    triangle.vertice_2.y *= 0.5 * HEIGHT as f32;
    triangle.vertice_3.x *= 0.5 * HEIGHT as f32;
    triangle.vertice_3.y *= 0.5 * HEIGHT as f32;
}
#[derive(Debug)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn divide_by(&mut self, val: f32) {
        self.x /= val;
        self.y /= val;
        self.z /= val;
    }

    fn add_z(&mut self, val: f32) {
        self.z += val;
    }
}

#[derive(Debug)]
struct Triangle {
    vertice_1: Vec3,
    vertice_2: Vec3,
    vertice_3: Vec3,
}

struct Mesh {
    triangles: Vec<Triangle>,
}

fn create_x_rot_mat(theta: &f32) -> Matrix4<f32> {
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
fn create_z_rot_mat(theta: &f32) -> Matrix4<f32> {
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

fn multiply_matrices(input_matrix: &Vec3, output_vec: &mut Vec3, proj_mat: &Matrix4<f32>) {
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

fn create_projection_matrix() -> Matrix4<f32> {
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

fn initialize_empty_triangle() -> Triangle {
    return Triangle {
        vertice_1: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        vertice_2: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        vertice_3: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    };
}

fn create_initial_triangles() -> Mesh {
    let mesh: Mesh = Mesh {
        triangles: vec![
            // south
            Triangle {
                vertice_1: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                vertice_2: Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                vertice_3: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                },
            },
            Triangle {
                vertice_1: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                vertice_2: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                },
                vertice_3: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            // east
            Triangle {
                vertice_1: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                vertice_2: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                },
                vertice_3: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            },
            Triangle {
                vertice_1: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                vertice_2: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                vertice_3: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
            },
            // north
            Triangle {
                vertice_1: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
                vertice_2: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                vertice_3: Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 1.0,
                },
            },
            Triangle {
                vertice_1: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
                vertice_2: Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 1.0,
                },
                vertice_3: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
            },
            // west
            Triangle {
                vertice_1: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                vertice_2: Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 1.0,
                },
                vertice_3: Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            },
            Triangle {
                vertice_1: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                vertice_2: Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                vertice_3: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            // top
            Triangle {
                vertice_1: Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                vertice_2: Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 1.0,
                },
                vertice_3: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            },
            Triangle {
                vertice_1: Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                vertice_2: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                vertice_3: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                },
            },
            // bottom
            Triangle {
                vertice_1: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
                vertice_2: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                vertice_3: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            Triangle {
                vertice_1: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
                vertice_2: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                vertice_3: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
        ],
    };

    mesh
}
