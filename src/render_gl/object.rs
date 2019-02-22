extern crate nalgebra_glm as glm;

use gl;
use std;

use crate::render_gl::data;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    pos: data::f32_f32_f32,
    clr: data::f32_f32_f32,
}

#[derive(Debug)]
pub struct Object {
    pub matrix: glm::Mat4,
    vertices: Vec<Vertex>,
    vert_array: gl::types::GLuint,
}

impl Object {
    pub fn make(
        gl: &gl::Gl,
        verts: Vec<(f32, f32, f32)>,
        colors: Vec<(f32, f32, f32)>,
    ) -> Result<Object, String> {
        let mut obj: Object = Object {
            matrix: glm::identity(),
            vertices: verts.iter().zip(colors.iter()).map(|(&vert_vec, &col_vec)| Vertex { pos: vert_vec.into(), clr: col_vec.into() }).collect(),
            vert_array: 0,
        };

        obj.gen_buffers(gl);

        Ok(obj)
    }

    pub fn draw(&self, gl: &gl::Gl) {
        unsafe {
            gl.BindVertexArray(self.vert_array);
            gl.DrawArrays(
                gl::TRIANGLES,
                0,
                self.vertices.len() as i32,
            );
        }
    }

    fn gen_buffers(&mut self, gl: &gl::Gl) {
        let mut vertex_buffer: gl::types::GLuint = 0;

        unsafe {
            gl.GenBuffers(1, &mut vertex_buffer);
        }

        unsafe {
            //                  v- we give it a buffer type that we want to work with
            gl.BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr, // size of data in bytes
                self.vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW // i guess this is how we want this buffer to be used?
            );
            gl.BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer.. looks like there's only a single buffer to interact with passing data at one time
        }

        unsafe {
            gl.GenVertexArrays(1, &mut self.vert_array);
            gl.BindVertexArray(self.vert_array);
            gl.BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
        }

        self.set_vertex_attrib_pointers(gl);

        unsafe {
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
            gl.BindVertexArray(0);
        }
    }

    fn set_vertex_attrib_pointers(&self, gl: &gl::Gl) {
        let stride = std::mem::size_of::<Vertex>(); // byte offset between consecutive attributes

        let location = 0; // layout (location = 0)
        let offset = 0; // offset for first component

        unsafe {
            self.set_vertex_attrib_pointer(gl, stride, location, offset);
        }

        let location = 1;
        let offset = offset + std::mem::size_of::<data::f32_f32_f32>();

        unsafe {
            self.set_vertex_attrib_pointer(gl, stride, location, offset);
        }
    }

    unsafe fn set_vertex_attrib_pointer(&self, gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3, // number of components per vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}