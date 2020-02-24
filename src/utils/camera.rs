use crate::shader_common::*;

#[allow(dead_code)]
pub struct Camera {
    near: f64,
    far: f64,
    fovy: cgmath::Deg<f64>,
    aspect: f64,
}

impl Camera {
    pub fn perspective_projection(&self) -> float4x4 {
        cgmath::perspective(self.fovy, self.aspect, self.near, self.far)
    }
}
