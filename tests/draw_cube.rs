use naivegl::pipeline::*;
use naivegl::types::*;
use rayon::prelude::*;

#[test]
fn draw_cube() {
    const SCR_WIDTH: usize = 800;
    const SCR_HEIGHT: usize = 800;

    #[rustfmt::skip]
    let positions: [f64; 8 * 3] = [
        0.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
        0.0, 1.0, 1.0,
        1.0, 0.0, 1.0,
        1.0, 1.0, 1.0,
    ];

    let indices: [usize; 3] = [0, 1, 2];

    let vin_vec: Vec<VShaderIn> = positions
        .par_iter()
        .enumerate()
        .filter_map(|(i, _)| {
            if i % 3 == 0 {
                let vertex = float4::new(positions[i], positions[i + 1], positions[i + 2], 1.0);
                let vin = VShaderIn {
                    vertex,
                    normal: None,
                    texcoord: None,
                    texcoord1: None,
                    tangent: None,
                    color: None,
                };
                Some(vin)
            } else {
                None
            }
        })
        .collect();

    let cube_vs = |vin: &VShaderIn| {
        let clip_pos = vin.vertex;
        let vert_color = None;
        let world_normal = None;
        let screen_pos = None;
        VShaderOut {
            clip_pos,
            screen_pos,
            vert_color,
            world_normal,
        }
    };

    let cube_fs = |fin: &FShaderIn| {
        let depth = fin.depth;
        let color = match fin.value.vert_color {
            Some(color) => color,
            None => float4::new(1.0, 1.0, 1.0, 1.0),
        };
        let screen_x = fin.screen_x;
        let screen_y = fin.screen_y;
        FShaderOut {
            depth,
            color,
            screen_x,
            screen_y,
        }
    };

    let mut fb = Framebuffer::new(SCR_WIDTH, SCR_HEIGHT);
    process_pipeline(&vin_vec, &indices, cube_vs, cube_fs, &mut fb);

    fb.write_image(std::path::Path::new("output/draw_cube.png"))
        .unwrap();
}
