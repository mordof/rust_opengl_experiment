extern crate nalgebra_glm as glm;

use gl;

use crate::render_gl::object::{Object};
use crate::render_gl::shader::{Program};

pub fn make_mountain(
    gl: &gl::Gl,
    program: Program,
    width: f32,
    depth: f32,
    height: f32,
    point_count: u32,
) -> Object {
    let width_space: f32 = width / point_count as f32;
    let depth_space: f32 = depth / point_count as f32;

    let mut points: Vec<(f32, f32, f32)> = Vec::new();
    let mut colors: Vec<(f32, f32, f32)> = Vec::new();

    for start_x in (0..point_count).map(|i| (i as f32 * width_space) - (width / 2.0)) {
        for start_z in (0..point_count).map(|i| (i as f32 * depth_space) - (depth / 2.0)) {
            for triangle in 0..2 {
                let mut tri_points: Vec<(f32, f32, f32)> = Vec::new();

                for vert in (triangle * 3)..((triangle * 3) + 3) {
                    let x = start_x + match triangle + vert {
                        1 | 2 | 4 => width_space,
                        _ => 0.0,
                    };
                    let z = start_z + match triangle + vert {
                        2 | 4 | 5 => depth_space,
                        _ => 0.0,
                    };
                    let y = get_y(x, z, width, depth, height);

                    tri_points.push((x, y, z));
                    points.push((x, y, z));
                }

                let avg_y: f32 = (tri_points[0].1 + tri_points[1].1 + tri_points[2].1) / 3.0;
                let avg_y_color: (f32, f32, f32) = get_color(avg_y, height);

                colors.push(avg_y_color);
                colors.push(avg_y_color);
                colors.push(avg_y_color);
            }
        }
    }

    println!("point, color len: ({:?}, {:?})", points.len(), colors.len());

    Object::make(
        gl,
        program,
        points,
        colors,
    )
}

fn get_y(x: f32, z: f32, width: f32, depth: f32, height: f32) -> f32 {
    let xy = glm::abs(&glm::vec1(x)).x; // 0.0..(width / 2.0)
    let zy = glm::abs(&glm::vec1(z)).x; // 0.0..(depth / 2.0)

    let xy = xy / (width / 2.0); // 0.0..1.0 (1.0 means outside, 0.0 is center)
    let zy = zy / (depth / 2.0); // 0.0..1.0 (1.0 means outside, 0.0 is center)

    let xy = glm::abs(&glm::vec1(1.0 - xy)).x; // invert: 0.0 is now outside, 1.0 is center
    let zy = glm::abs(&glm::vec1(1.0 - zy)).x; // invert: 0.0 is now outside, 1.0 is center

    let xy = xy * height; // width y value
    let zy = zy * height; // depth y value

    xy.min(zy)
}

fn get_color(y: f32, height: f32) -> (f32, f32, f32) {
    ((y / height) * 255.0, (y / height) * 255.0, (y / height) * 255.0)
}