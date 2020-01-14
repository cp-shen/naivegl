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
    return v2f_vec.to_owned();
}

/// TODO
pub fn perform_screen_mapping(v2f_vec: &[V2f]) -> Vec<V2f> {
    return v2f_vec.to_owned();
}

pub fn setup_triangle(v2f_vec: &[V2f], indices: &[u32]) -> Vec<V2f> {
    return v2f_vec.to_owned();
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
