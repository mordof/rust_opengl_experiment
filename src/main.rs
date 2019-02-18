#[macro_use]
extern crate failure;

extern crate sdl2;
extern crate gl;
extern crate nalgebra_glm as glm;

pub mod render_gl;
pub mod resources;

use crate::resources::Resources;
use std::path::Path;
use crate::render_gl::data;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    pos: data::f32_f32_f32,
    clr: data::f32_f32_f32,
}

impl Vertex {
    fn vertex_attrib_pointers(gl: &gl::Gl) {
        let stride = std::mem::size_of::<Self>(); // byte offset between consecutive attributes

        let location = 0; // layout (location = 0)
        let offset = 0; // offset for first component

        unsafe {
            data::f32_f32_f32::vertex_attrib_pointer(gl, stride, location, offset);
        }

        let location = 1;
        let offset = offset + std::mem::size_of::<data::f32_f32_f32>();

        unsafe {
            data::f32_f32_f32::vertex_attrib_pointer(gl, stride, location, offset);
        }
    }
}

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

    let shader_program = render_gl::Program::from_res(
        &gl, &res, "shaders/triangle"
    ).unwrap();

    let vertices: Vec<Vertex> = vec![
        // plane
        Vertex { pos: (0.5, 0.5, 0.0).into(), clr: (1.0, 0.0, 0.0).into() },
        Vertex { pos: (0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0).into() },
        Vertex { pos: (-0.5, 0.5, 0.0).into(), clr: (1.0, 0.0, 0.0).into() },

        Vertex { pos: (0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0).into() },
        Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0).into() },
        Vertex { pos: (-0.5, 0.5, 0.0).into(), clr: (1.0, 0.0, 0.0).into() },

        // cube
        // Vertex { pos: (-0.3, -0.3, -0.3).into(), clr: (1.0, 0.0, 0.0).into() },
        // Vertex { pos: ( 0.3, -0.3, -0.3).into(), clr: (1.0, 0.0, 0.0).into() },
        // Vertex { pos: ( 0.3,  0.3, -0.3).into(), clr: (1.0, 0.0, 0.0).into() },
        // Vertex { pos: ( 0.3,  0.3, -0.3).into(), clr: (1.0, 0.0, 0.0).into() },
        // Vertex { pos: (-0.3,  0.3, -0.3).into(), clr: (1.0, 0.0, 0.0).into() },
        // Vertex { pos: (-0.3, -0.3, -0.3).into(), clr: (1.0, 0.0, 0.0).into() },

        // Vertex { pos: (-0.3, -0.3,  0.3).into(), clr: (0.0, 1.0, 0.0).into() },
        // Vertex { pos: ( 0.3, -0.3,  0.3).into(), clr: (0.0, 1.0, 0.0).into() },
        // Vertex { pos: ( 0.3,  0.3,  0.3).into(), clr: (0.0, 1.0, 0.0).into() },
        // Vertex { pos: ( 0.3,  0.3,  0.3).into(), clr: (0.0, 1.0, 0.0).into() },
        // Vertex { pos: (-0.3,  0.3,  0.3).into(), clr: (0.0, 1.0, 0.0).into() },
        // Vertex { pos: (-0.3, -0.3,  0.3).into(), clr: (0.0, 1.0, 0.0).into() },

        // Vertex { pos: (-0.3,  0.3,  0.3).into(), clr: (0.0, 0.0, 1.0).into() },
        // Vertex { pos: (-0.3,  0.3, -0.3).into(), clr: (0.0, 0.0, 1.0).into() },
        // Vertex { pos: (-0.3, -0.3, -0.3).into(), clr: (0.0, 0.0, 1.0).into() },
        // Vertex { pos: (-0.3, -0.3, -0.3).into(), clr: (0.0, 0.0, 1.0).into() },
        // Vertex { pos: (-0.3, -0.3,  0.3).into(), clr: (0.0, 0.0, 1.0).into() },
        // Vertex { pos: (-0.3,  0.3,  0.3).into(), clr: (0.0, 0.0, 1.0).into() },

        // Vertex { pos: ( 0.3,  0.3,  0.3).into(), clr: (1.0, 1.0, 0.0).into() },
        // Vertex { pos: ( 0.3,  0.3, -0.3).into(), clr: (1.0, 1.0, 0.0).into() },
        // Vertex { pos: ( 0.3, -0.3, -0.3).into(), clr: (1.0, 1.0, 0.0).into() },
        // Vertex { pos: ( 0.3, -0.3, -0.3).into(), clr: (1.0, 1.0, 0.0).into() },
        // Vertex { pos: ( 0.3, -0.3,  0.3).into(), clr: (1.0, 1.0, 0.0).into() },
        // Vertex { pos: ( 0.3,  0.3,  0.3).into(), clr: (1.0, 1.0, 0.0).into() },

        // Vertex { pos: (-0.3, -0.3, -0.3).into(), clr: (1.0, 0.0, 1.0).into() },
        // Vertex { pos: ( 0.3, -0.3, -0.3).into(), clr: (1.0, 0.0, 1.0).into() },
        // Vertex { pos: ( 0.3, -0.3,  0.3).into(), clr: (1.0, 0.0, 1.0).into() },
        // Vertex { pos: ( 0.3, -0.3,  0.3).into(), clr: (1.0, 0.0, 1.0).into() },
        // Vertex { pos: (-0.3, -0.3,  0.3).into(), clr: (1.0, 0.0, 1.0).into() },
        // Vertex { pos: (-0.3, -0.3, -0.3).into(), clr: (1.0, 0.0, 1.0).into() },

        // Vertex { pos: (-0.3,  0.3, -0.3).into(), clr: (1.0, 1.0, 1.0).into() },
        // Vertex { pos: ( 0.3,  0.3, -0.3).into(), clr: (1.0, 1.0, 1.0).into() },
        // Vertex { pos: ( 0.3,  0.3,  0.3).into(), clr: (1.0, 1.0, 1.0).into() },
        // Vertex { pos: ( 0.3,  0.3,  0.3).into(), clr: (1.0, 1.0, 1.0).into() },
        // Vertex { pos: (-0.3,  0.3,  0.3).into(), clr: (1.0, 1.0, 1.0).into() },
        // Vertex { pos: (-0.3,  0.3, -0.3).into(), clr: (1.0, 1.0, 1.0).into() },
    ];

    let mut vertex_buffer: gl::types::GLuint = 0;

    unsafe {
        gl.GenBuffers(1, &mut vertex_buffer);
    }

    unsafe {
        //                  v- we give it a buffer type that we want to work with
        gl.BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW // i guess this is how we want this buffer to be used?
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer.. looks like there's only a single buffer to interact with passing data at one time
    }

    let mut vertex_array: gl::types::GLuint = 0;

    unsafe {
        gl.GenVertexArrays(1, &mut vertex_array);
    }

    unsafe {
        gl.BindVertexArray(vertex_array);
        gl.BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
    }

    Vertex::vertex_attrib_pointers(&gl);

    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

    unsafe {
        gl.Viewport(0, 0, SCR_WIDTH as i32, SCR_HEIGHT as i32); // set viewport
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let m4default = glm::mat4(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        let mut model = m4default;
        let mut view = m4default;
        let mut projection = m4default;

        model = glm::rotate(&model, glm::radians(&glm::vec1(-55.0)).x, &glm::vec3(1.0, 0.0, 0.0));
        view = glm::translate(&view, &glm::vec3(0.0, 0.0, -3.0));
        projection = glm::perspective(SCR_WIDTH as f32 / SCR_HEIGHT as f32, glm::radians(&glm::vec1(45.0)).x, 0.1, 100.0);

        shader_program.set_mat4(&gl, "model", &model);
        shader_program.set_mat4(&gl, "view", &view);
        shader_program.set_mat4(&gl, "projection", &projection);

        // mat4 * vec4
        // {{a, b, c, d}, {e, f, g, h}, {i, j, k, l}, {m, n, o, p}} * {w, x, y, z} = {aw + bx + cy + dz, ew + fx + gy + hz, iw + jx + ky + lz, mw + nx + oy + pz} 

        println!("model {:?}", model);
        println!("view {:?}", view);
        println!("projectoion {:?}", projection);

        shader_program.activate();

        unsafe {
            gl.BindVertexArray(vertex_array);
            gl.DrawArrays(
                gl::TRIANGLES,
                0,
                6
            );
        }

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