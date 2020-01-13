use std::option::Option;

mod types {
    pub type float4 = cgmath::Vector4<f64>;
    pub type float3 = cgmath::Vector3<f64>;
    pub type float2 = cgmath::Vector2<f64>;
}

use types::*;
struct Appdata {
    vertex: float3,
    normal: Option<float3>,
    texcoord: Option<float2>,
    texcoord1: Option<float2>,
    tangent: Option<float3>,
    color: Option<float4>,
}

struct V2f {
    pos: float3,
    color: Option<float4>,
}

fn process_vertices(vertices: Vec<Appdata>) -> Vec<V2f> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
