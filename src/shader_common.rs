use cgmath::prelude::*;
use std::option::Option;

#[allow(non_camel_case_types)]
pub type float4 = cgmath::Vector4<f64>;
#[allow(non_camel_case_types)]
pub type float3 = cgmath::Vector3<f64>;
#[allow(non_camel_case_types)]
pub type float2 = cgmath::Vector2<f64>;
#[allow(non_camel_case_types)]
pub type float4x4 = cgmath::Matrix4<f64>;
#[allow(non_camel_case_types)]
pub type float3x3 = cgmath::Matrix3<f64>;

#[derive(Clone)]
pub struct VShaderIn {
    pub vertex: float4,
    pub normal: Option<float4>,
    pub texcoord: Option<float2>,
    pub texcoord1: Option<float2>,
    pub tangent: Option<float4>,
    pub color: Option<float4>,
}

impl Default for VShaderIn {
    fn default() -> Self {
        VShaderIn {
            vertex: float4::zero(),
            normal: Default::default(),
            texcoord: Default::default(),
            texcoord1: Default::default(),
            tangent: Default::default(),
            color: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct VShaderOut {
    pub clip_pos: float4,
    pub screen_pos: Option<float3>,
    pub world_normal: Option<float4>,
    pub world_pos: Option<float4>,
    pub vert_color: Option<float4>,
}

impl Default for VShaderOut {
    fn default() -> Self {
        VShaderOut {
            clip_pos: float4::zero(),
            screen_pos: Default::default(),
            world_normal: Default::default(),
            world_pos: Default::default(),
            vert_color: Default::default(),
        }
    }
}

#[derive(Clone, Default)]
pub struct FShaderIn {
    pub screen_x: usize,
    pub screen_y: usize,
    pub depth: f64,
    /// interpolated value
    pub value: VShaderOut,
}

#[derive(Clone)]
pub struct FShaderOut {
    pub screen_x: usize,
    pub screen_y: usize,
    pub depth: f64,
    pub color: float4,
}

impl Default for FShaderOut {
    fn default() -> Self {
        FShaderOut {
            color: float4::zero(),
            screen_x: Default::default(),
            screen_y: Default::default(),
            depth: Default::default(),
        }
    }
}
