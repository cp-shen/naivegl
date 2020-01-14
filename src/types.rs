use std::option::Option;

pub type float4 = cgmath::Vector4<f64>;
pub type float3 = cgmath::Vector3<f64>;
pub type float2 = cgmath::Vector2<f64>;

pub type VertexShader = fn(&A2v) -> V2f;
pub type FragmentShader = fn(&V2f) -> Fout;

pub struct A2v {
    vertex: float3,
    normal: Option<float3>,
    texcoord: Option<float2>,
    texcoord1: Option<float2>,
    tangent: Option<float3>,
    color: Option<float4>,
}

#[derive(Clone)]
pub struct V2f {
    pos: float3,
    color: Option<float4>,
}

pub struct Fout {
    depth: f64,
    color: float4,
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
