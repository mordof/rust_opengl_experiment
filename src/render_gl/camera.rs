extern crate nalgebra_glm as glm;

use crate::render_gl::object::{Object};
use gl;

pub struct Camera {
    pub matrix: glm::Mat4,
    gl: gl::Gl,
    lense: glm::Mat4,
}

impl Camera {
    pub fn make(
        gl: &gl::Gl,
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
        }
    }

    pub fn draw(&self, obj: &Object) {
        obj.shader_program.set_mat4(&self.gl, "view", &self.matrix);
        obj.shader_program.set_mat4(&self.gl, "projection", &self.lense);

        obj.draw();
    }
}