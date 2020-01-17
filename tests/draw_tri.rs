use naivegl::pipeline::*;
use naivegl::types::*;
use rayon::prelude::*;

#[test]
fn draw_tri() {
    #[rustfmt::skip]
    let positions: Vec<f64> = vec![
        0.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        1.0, 0.0, 0.0,
    ];

    let appdate: Vec<VShaderIn> = positions
        .par_iter()
        .enumerate()
        .filter_map(|(i, _)| {
            if i % 3 == 0 {
                let vertex = float3::new(positions[i], positions[i + 1], positions[i + 2]);
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

    let tri_vs = |appdate: &VShaderIn| {
        let pos = appdate.vertex;
        let color = None;
        VShaderOut { pos, color }
    };

    const SCR_WIDTH: usize = 800;
    const SCR_HEIGHT: usize = 800;

    let v2f_vec = process_vertices(&appdate, tri_vs);
    let v2f_vec = perform_clipping(&v2f_vec);
    let v2f_vec = perform_screen_mapping(&v2f_vec, SCR_WIDTH, SCR_HEIGHT);

    let indices: [usize; 3] = [0, 1, 2];

    let v2f_vec = setup_triangle(&v2f_vec, &indices);

    let tri_fs = |v2f: &VShaderOut| {
        let depth = 0.0;
        let color = float4::new(1.0, 1.0, 1.0, 1.0);
        //TODO: compute screen space coords
        FShaderOut {
            depth,
            color,
            x: 0,
            y: 0,
        }
    };

    let fout_vec = process_fragments(&v2f_vec, tri_fs);
}
