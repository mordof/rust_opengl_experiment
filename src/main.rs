#[macro_use]
extern crate failure;

extern crate sdl2;
extern crate gl;
extern crate nalgebra_glm as glm;

mod render_gl;
mod resources;
mod generators;

use crate::resources::Resources;
use std::path::Path;

use crate::generators::mountain::{make_mountain};

use crate::render_gl::object::{Object};
use crate::render_gl::camera::{Camera};

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    if let Err(e) = run() {
        println!("{}", failure_to_string(e));
    }
}

fn run() -> Result<(), failure::Error> {
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Game", SCR_WIDTH, SCR_HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();

    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl.Enable(gl::DEPTH_TEST);
    }

    let mountain_program = render_gl::Program::from_res(
        &gl, &res, "shaders/triangle"
    ).unwrap();

    let square_program = render_gl::Program::from_res(
        &gl, &res, "shaders/triangle"
    ).unwrap();

    let mut camera: Camera = Camera::make(
        &gl,
        SCR_WIDTH,
        SCR_HEIGHT,
        45.0,
        0.1,
        1000.0,
    );

    
    camera.matrix = glm::translate(&camera.matrix, &glm::vec3(0.0, -100.0, -200.0));
    // camera.matrix = glm::rotate_x(&camera.matrix, glm::radians(&glm::vec1(10.0)).x);
    

    let mountain: Object = make_mountain(&gl, mountain_program, 100.0, 100.0, 45.0, 200);

    let square: Object = Object::make(
        &gl,
        square_program,
        vec![
            (-0.3, -0.3, -0.3), ( 0.3, -0.3, -0.3), ( 0.3,  0.3, -0.3),
            ( 0.3,  0.3, -0.3), (-0.3,  0.3, -0.3), (-0.3, -0.3, -0.3),

            (-0.3, -0.3,  0.3), ( 0.3, -0.3,  0.3), ( 0.3,  0.3,  0.3),
            ( 0.3,  0.3,  0.3), (-0.3,  0.3,  0.3), (-0.3, -0.3,  0.3),

            (-0.3,  0.3,  0.3), (-0.3,  0.3, -0.3), (-0.3, -0.3, -0.3),
            (-0.3, -0.3, -0.3), (-0.3, -0.3,  0.3), (-0.3,  0.3,  0.3),

            ( 0.3,  0.3,  0.3), ( 0.3,  0.3, -0.3), ( 0.3, -0.3, -0.3),
            ( 0.3, -0.3, -0.3), ( 0.3, -0.3,  0.3), ( 0.3,  0.3,  0.3),

            (-0.3, -0.3, -0.3), ( 0.3, -0.3, -0.3), ( 0.3, -0.3,  0.3),
            ( 0.3, -0.3,  0.3), (-0.3, -0.3,  0.3), (-0.3, -0.3, -0.3),

            (-0.3,  0.3, -0.3), ( 0.3,  0.3, -0.3), ( 0.3,  0.3,  0.3),
            ( 0.3,  0.3,  0.3), (-0.3,  0.3,  0.3), (-0.3,  0.3, -0.3),
        ],
        vec![
            (1.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0), (0.0, 1.0, 0.0), (0.0, 1.0, 0.0), (0.0, 1.0, 0.0), (0.0, 1.0, 0.0), (0.0, 1.0, 0.0),
            (0.0, 0.0, 1.0), (0.0, 0.0, 1.0), (0.0, 0.0, 1.0), (0.0, 0.0, 1.0), (0.0, 0.0, 1.0), (0.0, 0.0, 1.0),
            (1.0, 1.0, 0.0), (1.0, 1.0, 0.0), (1.0, 1.0, 0.0), (1.0, 1.0, 0.0), (1.0, 1.0, 0.0), (1.0, 1.0, 0.0),
            (1.0, 0.0, 1.0), (1.0, 0.0, 1.0), (1.0, 0.0, 1.0), (1.0, 0.0, 1.0), (1.0, 0.0, 1.0), (1.0, 0.0, 1.0),
            (1.0, 1.0, 1.0), (1.0, 1.0, 1.0), (1.0, 1.0, 1.0), (1.0, 1.0, 1.0), (1.0, 1.0, 1.0), (1.0, 1.0, 1.0),
        ],
    );

    // square.matrix = glm::rotate_x(&square.matrix, glm::radians(&glm::vec1(-55.0)).x);

    unsafe {
        gl.Viewport(0, 0, SCR_WIDTH as i32, SCR_HEIGHT as i32); // set viewport
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // square.matrix = glm::rotate_y(&square.matrix, glm::radians(&glm::vec1(2.0)).x);

        // camera.matrix = glm::translate(&camera.matrix, &glm::vec3(0.0, 0.0, -0.01));

        camera.draw(&square);
        camera.draw(&mountain);

        window.gl_swap_window();
    }

    Ok(())
}

pub fn failure_to_string(e: failure::Error) -> String {
    use std::fmt::Write;

    let mut result = String::new();

    for (i, cause) in e
        .iter_chain()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .enumerate()
    {
        if i > 0 {
            let _ = writeln!(&mut result, "   Which caused the following issue:");
        }
        let _ = write!(&mut result, "{}", cause);
        if let Some(backtrace) = cause.backtrace() {
            let backtrace_str = format!("{}", backtrace);
            if backtrace_str.len() > 0 {
                let _ = writeln!(&mut result, " This happened at {}", backtrace);
            } else {
                let _ = writeln!(&mut result);
            }
        } else {
            let _ = writeln!(&mut result);
        }
    }

    result
}