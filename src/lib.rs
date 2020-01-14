mod types;

use rayon::prelude::*;
use types::*;

pub fn process_vertices(
    a2v_vec: &[A2v],
    vs: VertexShader,
) -> Vec<V2f> {
    a2v_vec.par_iter().map(vs).collect()
}

/// TODO
pub fn perform_clipping(v2f_vec: &[V2f]) -> Vec<V2f> {
    return v2f_vec.to_owned();
}

/// TODO
pub fn perform_screen_mapping(v2f_vec: &[V2f]) -> Vec<V2f> {
    return v2f_vec.to_owned();
}

pub fn setup_triangle(v2f_vec: &[V2f], indices: &[u32]) -> Vec<V2f> {
    return v2f_vec.to_owned();
}

pub fn process_fragments(
    v2f_vec: &[V2f],
    fs: FragmentShader,
) -> Vec<Fout> {
    v2f_vec.par_iter().map(fs).collect()
}

pub fn merge_output() {
    unimplemented!();
}
