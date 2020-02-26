use naivegl::framebuffer::*;
use naivegl::pipeline::*;
use naivegl::shader_common::*;
use naivegl::utils::*;
use rayon::prelude::*;

#[test]
fn draw_cube() {
    const SCR_WIDTH: usize = 800;
    const SCR_HEIGHT: usize = 800;

    #[rustfmt::skip]
    let positions: [f64; 8 * 3] = [
        0.0, 0.0, 0.0,
        0.0, 0.5, 0.0,
        0.5, 0.0, 0.0,
        0.5, 0.5, 0.0,
        0.0, 0.0, 0.5,
        0.0, 0.5, 0.5,
        0.5, 0.0, 0.5,
        0.5, 0.5, 0.5,
    ];

    #[rustfmt::skip]
    let indices = [
        0, 4, 1,
        4, 5, 1,
        4, 7, 5,
        4, 6, 7,
        6, 3, 7,
        6, 2, 3,
        0, 1, 3,
        3, 2, 0,
        1, 5, 7,
        1, 7, 3,
        0, 2, 6,
        0, 4, 4,
    ];

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

    let model_scale = float4x4::from_scale(1.0);
    let model_rot = float4x4::from_angle_y(cgmath::Deg(10.0));
    let model_translation = float4x4::from_translation(cgmath::vec3(0.0, 0.0, 0.0));
    let model_matrix = model_translation * model_rot * model_scale;

    let view_matrix = float4x4::look_at_dir(
        cgmath::Point3::new(0.0, 1.5, 2.5),
        float3::new(0.0, -0.4, -1.0),
        float3::new(0.0, 1.0, 0.0),
    );

    let projection_matrix: float4x4 = cgmath::perspective(
        cgmath::Deg::<f64>(50.0),
        SCR_WIDTH as f64 / SCR_HEIGHT as f64,
        0.1,
        100.0,
    );

    let mvp = projection_matrix * view_matrix * model_matrix;

    let cube_vs = |vin: &VShaderIn| {
        let clip_pos = mvp * vin.vertex;
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
