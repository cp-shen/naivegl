use crate::shader_common::*;

#[allow(dead_code)]
pub struct Transform {
    pub translation: float3,
    pub scale: float3,
    pub rotation: cgmath::Quaternion<f64>,
}

impl Transform {
    pub fn to_matrix(&self) -> float4x4 {
        let scale = float4x4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        let rot = float4x4::from(self.rotation);
        let translation = float4x4::from_translation(self.translation);
        translation * rot * scale
    }
}
