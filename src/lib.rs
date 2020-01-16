pub mod types;

use rayon::prelude::*;
use std::marker::{Send, Sync};
use types::*;

pub fn process_vertices<VS>(a2v_vec: &[A2v], vs: VS) -> Vec<V2f>
where
    VS: Fn(&A2v) -> V2f + Send + Sync,
{
    a2v_vec.par_iter().map(vs).collect()
}

/// TODO
pub fn perform_clipping(v2f_vec: &[V2f]) -> Vec<V2f> {
    v2f_vec.to_owned()
}

pub fn perform_screen_mapping(
    v2f_vec: &[V2f],
    width: usize,
    height: usize,
) -> Vec<V2f> {
    v2f_vec
        .par_iter()
        .map(|v2f: &V2f| {
            let mut v2f_out = v2f.to_owned();
            v2f_out.pos.x *= width as f64 / 2.0;
            v2f_out.pos.y *= height as f64 / 2.0;
            v2f_out
        })
        .collect()
}

pub fn setup_triangle(v2f_vec: &[V2f], indices: &[u32]) -> Vec<V2f> {
    assert_eq!(
        indices.len() % 3,
        0,
        "indices length should be dividable by 3"
    );

    let v2f_out: Vec<V2f> = Vec::new();

    indices
        .par_iter()
        .enumerate()
        .filter_map(|(i, _)| {
            if i % 3 == 0 {
                Some((indices[i], indices[i + 1], indices[i + 2]))
            } else {
                None
            }
        })
        .map(|(i0, i1, i2)| {
            //TODO
            let color = float4::new(0.0, 0.0, 0.0, 0.0);
            let color = Some(color);
            let pos = float3::new(0.0, 0.0, 0.0);
            V2f { color, pos }
        })
        .collect()
}

pub fn process_fragments<FS>(v2f_vec: &[V2f], fs: FS) -> Vec<Fout>
where
    FS: Fn(&V2f) -> Fout + Send + Sync,
{
    v2f_vec.par_iter().map(fs).collect()
}

pub fn merge_output() {
    unimplemented!();
}
