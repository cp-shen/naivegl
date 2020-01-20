use naivegl::pipeline::*;
use naivegl::types::*;
use rayon::prelude::*;

#[test]
fn draw_tri() {
    #[rustfmt::skip]
    let positions: Vec<f64> = vec![
        -1.0, -1.0, 0.0,
        0.0, 1.0, 0.0,
        1.0, 0.0, 0.0,
    ];

    let vin_vec: Vec<VShaderIn> = positions
        .par_iter()
        .enumerate()
        .filter_map(|(i, _)| {
            if i % 3 == 0 {
                let vertex = float4::new(positions[i], positions[i + 1], positions[i + 2], 1.0);
                let vShaderIn = VShaderIn {
                    vertex,
                    normal: None,
                    texcoord: None,
                    texcoord1: None,
                    tangent: None,
                    color: None,
                };
                Some(vShaderIn)
            } else {
                None
            }
        })
        .collect();

    let tri_vs = |vin: &VShaderIn| {
        let clipPos = vin.vertex;
        let vertColor = None;
        let worldNormal = None;
        let screenPos = None;
        VShaderOut {
            clipPos,
            screenPos,
            vertColor,
            worldNormal,
        }
    };

    const SCR_WIDTH: usize = 800;
    const SCR_HEIGHT: usize = 800;

    let vout_vec = process_vertices(&vin_vec, tri_vs);
    let vout_vec_clipped = perform_clipping(&vout_vec);
    let vout_vec_mapped = perform_screen_mapping(&vout_vec_clipped, SCR_WIDTH, SCR_HEIGHT);
    let indices: [usize; 3] = [0, 1, 2];

    let fin_vec = setup_triangle(&vout_vec_mapped, &indices);

    let tri_fs = |fin: &FShaderIn| {
        let depth = fin.depth;
        let color = match fin.value.vertColor {
            Some(color) => color,
            None => float4::new(1.0, 1.0, 1.0, 1.0),
        };
        let screenX = fin.screenX;
        let screenY = fin.screenY;
        FShaderOut {
            depth,
            color,
            screenX,
            screenY,
        }
    };

    let fout_vec = process_fragments(&fin_vec, tri_fs);

    let mut fb = Framebuffer::new(SCR_WIDTH, SCR_HEIGHT);

    merge_output(&fout_vec, &mut fb);

    fb.write_image(std::path::Path::new("output/draw_tri.png"))
        .unwrap();
}
