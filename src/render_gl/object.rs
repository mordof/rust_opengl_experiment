extern crate nalgebra_glm as glm;

use gl;
use crate::render_gl::data;
use crate::render_gl::shader::{Program};

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    pos: data::f32_f32_f32,
    clr: data::f32_f32_f32,
}

pub struct Object {
    pub matrix: glm::Mat4,
    pub shader_program: Program,
    vertices: Vec<Vertex>,
    vert_array: gl::types::GLuint,
    gl: gl::Gl,
}

impl Object {
    pub fn make(
        gl: &gl::Gl,
        program: Program,
        verts: Vec<(f32, f32, f32)>,
        colors: Vec<(f32, f32, f32)>,
    ) -> Object {
        let mut obj: Object = Object {
            matrix: glm::identity(),
            vertices: verts.iter().zip(colors.iter()).map(|(&vert, &col)| Vertex { pos: vert.into(), clr: (col.0 / 255.0, col.1 / 255.0, col.2 / 255.0).into() }).collect(),
            vert_array: 0,
            shader_program: program,
            gl: gl.clone(),
        };

        obj.gen_buffers();

        obj
    }

    pub fn draw(&self) {
        self.shader_program.set_mat4(&self.gl, "model", &self.matrix);

        self.shader_program.activate();

        unsafe {
            self.gl.BindVertexArray(self.vert_array);
            self.gl.DrawArrays(
                gl::TRIANGLES,
                0,
                self.vertices.len() as i32,
            );
        }
    }

    fn gen_buffers(&mut self) {
        let mut vertex_buffer: gl::types::GLuint = 0;

        unsafe {
            self.gl.GenBuffers(1, &mut vertex_buffer);
        }

        unsafe {
            //                  v- we give it a buffer type that we want to work with
            self.gl.BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
            self.gl.BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr, // size of data in bytes
                self.vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW // i guess this is how we want this buffer to be used?
            );
            self.gl.BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer.. looks like there's only a single buffer to interact with passing data at one time
        }

        unsafe {
            self.gl.GenVertexArrays(1, &mut self.vert_array);
            self.gl.BindVertexArray(self.vert_array);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
        }

        self.set_vertex_attrib_pointers();

        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, 0);
            self.gl.BindVertexArray(0);
        }
    }

    fn set_vertex_attrib_pointers(&self) {
        let stride = std::mem::size_of::<Vertex>(); // byte offset between consecutive attributes

        let location = 0; // layout (location = 0)
        let offset = 0; // offset for first component

        unsafe {
            self.set_vertex_attrib_pointer(stride, location, offset);
        }

        let location = 1;
        let offset = offset + std::mem::size_of::<data::f32_f32_f32>();

        unsafe {
            self.set_vertex_attrib_pointer(stride, location, offset);
        }
    }

    unsafe fn set_vertex_attrib_pointer(&self, stride: usize, location: usize, offset: usize) {
        self.gl.EnableVertexAttribArray(location as gl::types::GLuint);
        self.gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3, // number of components per vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}