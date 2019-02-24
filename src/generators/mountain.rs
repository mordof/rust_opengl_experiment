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
            for vert in 0..6 {
                let x = start_x + match vert {
                    1 | 2 | 4 => width_space,
                    _ => 0.0,
                };
                let z = start_z + match vert {
                    2 | 4 | 5 => depth_space,
                    _ => 0.0,
                };
                let y = get_y(x, z, width, depth, height);

                points.push((x, y, z));
                colors.push(get_color(y, height));
            }
        }
    }

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

    if xy == 0.0 || zy == 0.0 {
        return 0.0;
    }
    
    // average the two
    (xy + zy) / 2.0
}

fn get_color(y: f32, height: f32) -> (f32, f32, f32) {
    if y / height < 0.4 {
        return (52.0, 175.0, 45.0);
    }

    if y / height < 0.7 {
        return (41.0, 52.0, 61.0);
    }

    (227.0, 239.0, 249.0)
}