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
        .iter()
        .enumerate()
        .step_by(3)
        .map(|(i, _)| {
            let vertex = float3::new(
                positions[i],
                positions[i + 1],
                positions[i + 2],
            );
            A2v {
                vertex,
                normal: None,
                texcoord: None,
                texcoord1: None,
                tangent: None,
                color: None,
            }
        })
        .collect();

    let tri_vs = |appdate: &A2v| {
        let pos = appdate.vertex;
        let color = Some(float4::new(1.0, 1.0, 1.0, 1.0));
        V2f { pos, color }
    };

    let v2f_vec = naivegl::process_vertices(&appdate, tri_vs);
    let v2f_vec = naivegl::perform_clipping(&v2f_vec);
    //let v2f_vec = naivegl::perform_screen_mapping(&v2f_vec);
}