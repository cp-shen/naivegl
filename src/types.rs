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
    pub clip_pos: float4,
    pub screen_pos: Option<float4>,
    pub world_normal: Option<float4>,
    pub vert_color: Option<float4>,
}

#[derive(Clone)]
pub struct FShaderIn {
    pub screen_x: usize,
    pub screen_y: usize,
    pub depth: f64,
    pub value: VShaderOut,
}

#[derive(Clone)]
pub struct FShaderOut {
    pub screen_x: usize,
    pub screen_y: usize,
    pub depth: f64,
    pub color: float4,
}

pub struct Framebuffer {
    width: usize,
    height: usize,
    ///Rgba Color
    color: Vec<(u8, u8, u8, u8)>,
}

impl Framebuffer {
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn new(width: usize, height: usize) -> Framebuffer {
        let size = width.checked_mul(height).unwrap();
        let color = vec![(0, 0, 0, 0); size];
        Framebuffer {
            width,
            height,
            color,
        }
    }

    pub fn write_image(&mut self, path: &std::path::Path) -> std::io::Result<()> {
        let imgbuf: image::RgbaImage =
            image::ImageBuffer::from_fn(self.width as u32, self.height as u32, |x, y| {
                let location_x = x as usize;
                let location_y = (self.height - 1 - y as usize)
                    .checked_mul(self.width)
                    .unwrap();
                let color = self.color[location_x.checked_add(location_y).unwrap()];
                image::Rgba([color.0, color.1, color.2, color.3])
            });

        imgbuf.save(path)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: (u8, u8, u8, u8)) {
        self.color[x + y * self.height] = color;
    }
}
