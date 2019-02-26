extern crate nalgebra_glm as glm;

use crate::render_gl::object::{Object};
use gl;

pub struct Camera {
    pub matrix: glm::Mat4,
    pub position: glm::Vec3,
    gl: gl::Gl,
    lense: glm::Mat4,
    up_direction: glm::Vec3,
    target: glm::Vec3,
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

        // let camera_facing = glm::normalize(&(camera_position - camera_target));
        // let camera_right = glm::normalize(&glm::cross::<f32, glm::U3>(&up_direction, &camera_facing));
        // let camera_up = glm::cross::<f32, glm::U3>(&camera_facing, &camera_right);

        Camera{
            up_direction,
            position: camera_position,
            target: camera_target,
            gl: gl.clone(),
            matrix: glm::look_at(&camera_position, &camera_target, &up_direction),
            lense: glm::perspective(width as f32 / height as f32, glm::radians(&glm::vec1(angle)).x, near, far),
        }
    }

    #[allow(dead_code)]
    pub fn look_at(&mut self, target: &glm::Vec3) {
        self.target = glm::vec3(target.x, target.y, target.z);
        self.matrix = glm::look_at(&self.position, target, &self.up_direction);
    }

    #[allow(dead_code)]
    pub fn reposition(&mut self, position: &glm::Vec3) {
        self.position = glm::vec3(position.x, position.y, position.z);
        self.matrix = glm::look_at(&self.position, &self.target, &self.up_direction);
    }

    #[allow(dead_code)]
    pub fn reposition_and_look_at(&mut self, position: &glm::Vec3, target: &glm::Vec3) {
        self.target = glm::vec3(target.x, target.y, target.z);
        self.position = glm::vec3(position.x, position.y, position.z);
        self.matrix = glm::look_at(&self.position, &self.target, &self.up_direction);
    }

    pub fn draw(&self, obj: &Object) {
        obj.shader_program.set_mat4(&self.gl, "view", &self.matrix);
        obj.shader_program.set_mat4(&self.gl, "projection", &self.lense);

        obj.draw();
    }
}