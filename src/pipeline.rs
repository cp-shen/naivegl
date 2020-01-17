#![allow(dead_code)]

use crate::math::*;
use crate::types::*;
use cgmath::Vector2;
use rayon::prelude::*;
use std::marker::{Send, Sync};

pub fn process_vertices<VS>(a2v_vec: &[A2v], vs: VS) -> Vec<V2f>
where
    VS: Fn(&A2v) -> V2f + Send + Sync,
{
    a2v_vec.par_iter().map(vs).collect()
}

pub fn perform_clipping(v2f_vec: &[V2f]) -> Vec<V2f> {
    //TODO: perform_clipping
    v2f_vec.to_owned()
}

pub fn perform_screen_mapping(v2f_vec: &[V2f], width: usize, height: usize) -> Vec<V2f> {
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

pub fn setup_triangle(v2f_vec: &[V2f], indices: &[usize]) -> Vec<V2f> {
    assert_eq!(
        indices.len() % 3,
        0,
        "indices length should be dividable by 3"
    );

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
            let mut v0 = v2f_vec[i0].pos;
            let mut v1 = v2f_vec[i1].pos;
            let v2 = v2f_vec[i2].pos;

            let val = (v1.y - v0.y) * (v2.x - v1.x) - (v1.x - v0.x) * (v2.y - v1.y);
            if val == 0.0 {
                panic!("the 3 vertices of a triangle should not be colinear");
            } else if val > 0.0 {
                std::mem::swap(&mut v0, &mut v1);
                // the 3 vertices should be counter clock-wise
            }

            // Compute triangle bounding box
            let minX: i64 = v0.x.min(v1.x).min(v2.x).floor() as i64;
            let minY: i64 = v0.x.min(v1.x).min(v2.y).floor() as i64;
            let maxX: i64 = v0.y.max(v1.y).max(v2.y).floor() as i64;
            let maxY: i64 = v0.y.max(v1.y).max(v2.y).floor() as i64;

            let edge01 = v1 - v0;
            let edge02 = v2 - v0;
            let edge12 = v2 - v1;

            let mut output: Vec<V2f> = Vec::new();

            for x in minX..maxX {
                for y in minY..maxY {
                    let mut overlaps = true;

                    let p_center = (x as f64 + 0.5, y as f64 + 0.5);
                    let w0 = edgeFn((v1.x, v1.y), (v2.x, v2.y), p_center);
                    let w1 = edgeFn((v2.x, v2.y), (v0.x, v0.y), p_center);
                    let w2 = edgeFn((v0.x, v0.y), (v1.x, v1.y), p_center);

                    overlaps &= if w0 == 0.0 {
                        (edge12.y == 0.0 && edge12.x < 0.0) || edge12.y < 0.0
                    } else {
                        w0 > 0.0
                    };
                    overlaps &= if w1 == 0.0 {
                        (edge02.y == 0.0 && edge02.x < 0.0) || edge02.y < 0.0
                    } else {
                        w1 > 0.0
                    };
                    overlaps &= if w2 == 0.0 {
                        (edge01.y == 0.0 && edge01.x < 0.0) || edge01.y < 0.0
                    } else {
                        w2 > 0.0
                    };

                    if overlaps {
                        //TODO: add interpolation
                        let color = float4::new(1.0, 1.0, 1.0, 1.0);
                        let color = Some(color);
                        let pos = float3::new(p_center.0, p_center.1, 0.0);
                        let v2f = V2f { color, pos };
                        output.push(v2f)
                    }
                }
            }

            output
        })
        .flatten()
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
