use crate::shader_common::*;
use cgmath::prelude::*;

#[allow(dead_code)]
pub struct Camera {
    pub near: f64,
    pub far: f64,
    pub fovy: cgmath::Deg<f64>,
    pub aspect: f64,
    pub position: float3,
    pub rotation: cgmath::Quaternion<f64>,
}

impl Camera {
    /// right-handed
    pub fn perspective_projection(&self) -> float4x4 {
        cgmath::perspective(self.fovy, self.aspect, self.near, self.far)
    }

    pub fn view_matrix(&self) -> float4x4 {
        let forward = self.rotation * float3::new(0.0, 0.0, 1.0);
        let target = self.position + forward;
        let up = self.rotation * float3::new(0.0, 1.0, 0.0);

        float4x4::look_at(
            cgmath::Point3::from_vec(self.position),
            cgmath::Point3::from_vec(target),
            up,
        )
    }
}
