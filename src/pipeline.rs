#![allow(dead_code)]

use crate::math::*;
use crate::types::*;
use rayon::prelude::*;
use std::marker::{Send, Sync};

pub fn process_vertices<VS>(vin_vec: &[VShaderIn], vs: VS) -> Vec<VShaderOut>
where
    VS: Fn(&VShaderIn) -> VShaderOut + Send + Sync,
{
    vin_vec.par_iter().map(vs).collect()
}

pub fn perform_clipping(vout_vec: &[VShaderOut]) -> Vec<VShaderOut> {
    //TODO: perform_clipping
    vout_vec.to_owned()
}

pub fn perform_screen_mapping(
    vout_vec: &[VShaderOut],
    width: usize,
    height: usize,
) -> Vec<VShaderOut> {
    vout_vec
        .par_iter()
        .map(|vout: &VShaderOut| {
            let mut vout_mapped = vout.to_owned();

            //TODO: implement screen mapping
            let screenPos = vout_mapped.clipPos.to_owned();
            screenPos.x *= width as f64 / 2.0;
            screenPos.y *= height as f64 / 2.0;

            vout_mapped.screenPos = Some(screenPos);
            vout_mapped
        })
        .collect()
}

pub fn setup_triangle(v2f_vec: &[VShaderOut], indices: &[usize]) -> Vec<FShaderIn> {
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

            let mut output: Vec<VShaderOut> = Vec::new();

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
                        let v2f = VShaderOut { color, pos };
                        output.push(v2f)
                    }
                }
            }

            output
        })
        .flatten()
        .collect()
}

pub fn process_fragments<FS>(fin_vec: &[FShaderIn], fs: FS) -> Vec<FShaderOut>
where
    FS: Fn(&VShaderOut) -> FShaderOut + Send + Sync,
{
    fin_vec.par_iter().map(fs).collect()
}

pub fn merge_output(fout_vec: &[FShaderOut], fb: &mut Framebuffer) {
    // TODO
    fout_vec.par_iter().for_each(|fout| {})
}
