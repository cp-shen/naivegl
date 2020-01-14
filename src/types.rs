use std::marker::{Send, Sync};
use std::option::Option;

pub type float4 = cgmath::Vector4<f64>;
pub type float3 = cgmath::Vector3<f64>;
pub type float2 = cgmath::Vector2<f64>;

pub struct A2v {
    pub vertex: float3,
    pub normal: Option<float3>,
    pub texcoord: Option<float2>,
    pub texcoord1: Option<float2>,
    pub tangent: Option<float3>,
    pub color: Option<float4>,
}

#[derive(Clone)]
pub struct V2f {
    pub pos: float3,
    pub color: Option<float4>,
}

pub struct Fout {
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
