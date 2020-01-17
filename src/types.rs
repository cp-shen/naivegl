use std::marker::{Send, Sync};
use std::option::Option;

#[allow(non_camel_case_types)]
pub type float4 = cgmath::Vector4<f64>;
#[allow(non_camel_case_types)]
pub type float3 = cgmath::Vector3<f64>;
#[allow(non_camel_case_types)]
pub type float2 = cgmath::Vector2<f64>;

#[derive(Clone)]
pub struct VShaderIn {
    pub vertex: float4,
    pub normal: Option<float4>,
    pub texcoord: Option<float2>,
    pub texcoord1: Option<float2>,
    pub tangent: Option<float4>,
    pub color: Option<float4>,
}

#[derive(Clone)]
pub struct VShaderOut {
    pub clipPos: float4,
    pub screenPos: Option<float4>,
    pub worldNormal: Option<float4>,
    pub vertColor: Option<float4>,
}

#[derive(Clone)]
pub struct FShaderIn {
    pub screenX: usize,
    pub screenY: usize,
    pub depth: f64,
    pub value: VShaderOut,
}

#[derive(Clone)]
pub struct FShaderOut {
    pub screenX: usize,
    pub screenY: usize,
    pub depth: f64,
    pub color: float4,
}

pub struct Framebuffer {
    width: usize,
    height: usize,
    color: Vec<u8>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        let size = width.checked_mul(height).unwrap();
        Framebuffer {
            width,
            height,
            color: Vec::with_capacity(size),
        }
    }

    pub fn write_image(&mut self, path: &std::path::Path) {
        unimplemented!();
    }
}
