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
