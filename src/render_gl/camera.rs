extern crate nalgebra_glm as glm;

use crate::render_gl::object::{Object};
use crate::render_gl::shader::{Program};
use gl;

pub struct Camera {
    pub matrix: glm::Mat4,
    gl: gl::Gl,
    lense: glm::Mat4,
    shader_program: Program,
}

impl Camera {
    pub fn make(
        gl: &gl::Gl,
        program: Program,
        width: u32,
        height: u32,
        angle: f32,
        near: f32,
        far: f32,
    ) -> Camera {
        Camera{
            gl: gl.clone(),
            matrix: glm::translate(&glm::identity(), &glm::vec3(0.0, 0.0, -3.0)),
            lense: glm::perspective(width as f32 / height as f32, glm::radians(&glm::vec1(angle)).x, near, far),
            shader_program: program,
        }
    }

    pub fn activate(&self) {
        self.shader_program.activate();
    }

    pub fn draw(&self, obj: &Object) {
        self.shader_program.set_mat4(&self.gl, "model", &obj.matrix);
        self.shader_program.set_mat4(&self.gl, "view", &self.matrix);
        self.shader_program.set_mat4(&self.gl, "projection", &self.lense);

        obj.draw(&self.gl);
    }
}