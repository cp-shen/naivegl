use naivegl::framebuffer::*;
use naivegl::pipeline::*;
use naivegl::shader_common::*;
#[allow(unused_imports)]
use naivegl::utils::*;
use rayon::prelude::*;

#[test]
fn draw_tri_3d() {
    const SCR_WIDTH: usize = 801;
    const SCR_HEIGHT: usize = 801;

    #[rustfmt::skip]
    let positions: [f64; 9] = [
        0.0, 0.0, 0.0,
        0.5, 0.0, 0.0,
        0.0, 0.5, 0.0,
    ];

    let indices = [0, 1, 2];

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

    let model_rot = float4x4::from_angle_y(cgmath::Rad(0.0));
    let model_translation = float4x4::from_translation(cgmath::vec3(0.0, 0.0, 0.0));
    let model_matrix = model_translation * model_rot * model_scale;

    let view_matrix = float4x4::look_at_dir(
        cgmath::Point3::new(0.0, 0.0, 5.0),
        float3::new(0.0, 0.0, -1.0),
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

        VShaderOut {
            clip_pos,
            ..Default::default()
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
    let vout_vec = process_vertices(&vin_vec, cube_vs);
    let vout_vec_clipped = perform_clipping(&vout_vec);
    let vout_vec_mapped =
        perform_screen_mapping(&vout_vec_clipped, fb.get_width(), fb.get_height());
    let fin_vec = setup_triangle(&vout_vec_mapped, &indices);
    let fout_vec = process_fragments(&fin_vec, cube_fs);
    merge_output(&fout_vec, &mut fb);

    for i in 0..vout_vec.len() {
        let model_pos = vin_vec[i].vertex;
        let clip_pos = vout_vec[i].clip_pos;
        let screen_pos = vout_vec_mapped[i].screen_pos.unwrap();
        println!(
            "model_pos: {:3.4}, {:3.4}, {:3.4}, {:3.4}\
           \nclip_pos:  {:3.4}, {:3.4}, {:3.4}, {:3.4}\
           \nscreen_pos:{:3.4}, {:3.4}, {:3.4}, \n",
            model_pos.x,
            model_pos.y,
            model_pos.z,
            model_pos.w,
            clip_pos.x,
            clip_pos.y,
            clip_pos.z,
            clip_pos.w,
            screen_pos.x,
            screen_pos.y,
            screen_pos.z,
        );
    }

    fb.write_image(std::path::Path::new("output/draw_tri_3d.png"))
        .unwrap();
}
