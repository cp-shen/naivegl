use crate::shader_common::*;

#[allow(dead_code)]
pub struct Transform {
    position: float3,
    scale: float3,
    rotation: cgmath::Quaternion<f64>,
}
