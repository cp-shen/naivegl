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

    let appdate: Vec<A2v> = positions
        .par_iter()
        .enumerate()
        .filter_map(|(i, _)| {
            if i % 3 == 0 {
                let vertex = float3::new(
                    positions[i],
                    positions[i + 1],
                    positions[i + 2],
                );
                let a2v = A2v {
                    vertex,
                    normal: None,
                    texcoord: None,
                    texcoord1: None,
                    tangent: None,
                    color: None,
                };
                Some(a2v)
            } else {
                None
            }
        })
        .collect();

    let tri_vs = |appdate: &A2v| {
        let pos = appdate.vertex;
        let color = None;
        V2f { pos, color }
    };

    const SCR_WIDTH: usize = 800;
    const SCR_HEIGHT: usize = 800;

    let v2f_vec = naivegl::process_vertices(&appdate, tri_vs);
    let v2f_vec = naivegl::perform_clipping(&v2f_vec);
    let v2f_vec = naivegl::perform_screen_mapping(
        &v2f_vec, SCR_WIDTH, SCR_HEIGHT,
    );

    let indices: [u32; 3] = [0, 1, 2];

    let v2f_vec = naivegl::setup_triangle(&v2f_vec, &indices);
}
