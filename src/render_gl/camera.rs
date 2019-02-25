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
        let up_direction = glm::vec3(0.0, 1.0, 0.0);
    
        let camera_position = glm::vec3(0.0, 0.0, -3.0);
        let camera_target = glm::vec3(0.0, 0.0, 0.0);

        let camera_facing = glm::normalize(&(camera_position - camera_target));
        let camera_right = glm::normalize(&glm::cross::<f32, glm::U3>(&up_direction, &camera_facing));
        let camera_up = glm::cross::<f32, glm::U3>(&camera_facing, &camera_right);

        

        Camera{
            gl: gl.clone(),
            matrix: glm::translate(&glm::identity(), &camera_position),
            lense: glm::perspective(width as f32 / height as f32, glm::radians(&glm::vec1(angle)).x, near, far),
        }
    }

    pub fn draw(&self, obj: &Object) {
        obj.shader_program.set_mat4(&self.gl, "view", &self.matrix);
        obj.shader_program.set_mat4(&self.gl, "projection", &self.lense);

        obj.draw();
    }
}