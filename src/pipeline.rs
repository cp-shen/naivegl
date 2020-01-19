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
    //TODO: perform clipping
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

            //TODO: transform clipPos to NDC

            //transform NDC to screenPos
            let screenPos = vout_mapped.clipPos;
            screenPos.x += 1.0;
            screenPos.y += 1.0;
            screenPos.x *= width as f64 / 2.0;
            screenPos.y *= height as f64 / 2.0;

            vout_mapped.screenPos = Some(screenPos);
            vout_mapped
        })
        .collect()
}

pub fn setup_triangle(vout_vec: &[VShaderOut], indices: &[usize]) -> Vec<FShaderIn> {
    assert_eq!(
        indices.len() % 3,
        0,
        "length of indices should be dividable by 3"
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
            let mut v0 = vout_vec[i0].screenPos.unwrap();
            let mut v1 = vout_vec[i1].screenPos.unwrap();
            let v2 = vout_vec[i2].screenPos.unwrap();

            let val = (v1.y - v0.y) * (v2.x - v1.x) - (v1.x - v0.x) * (v2.y - v1.y);

            if val == 0.0 {
                panic!("the 3 vertices of a triangle should not be colinear");
            } else if val > 0.0 {
                //the 3 vertices should be counter clock-wise
                std::mem::swap(&mut v0, &mut v1);
            }

            //Compute triangle bounding box
            let minX: usize = v0.x.min(v1.x).min(v2.x).floor() as usize;
            let minY: usize = v0.x.min(v1.x).min(v2.y).floor() as usize;
            let maxX: usize = v0.y.max(v1.y).max(v2.y).floor() as usize;
            let maxY: usize = v0.y.max(v1.y).max(v2.y).floor() as usize;

            let tri2d = Triangle2d {
                x0: v0.x,
                y0: v0.y,
                x1: v1.x,
                y1: v1.y,
                x2: v2.x,
                y2: v2.y,
            };

            let mut output: Vec<FShaderIn> = Vec::new();

            for x in minX..maxX {
                for y in minY..maxY {
                    let p_center = (x as f64 + 0.5, y as f64 + 0.5);

                    if tri2d.overlaps_point(p_center.0, p_center.1) {
                        //TODO: add interpolation
                        let color = float4::new(1.0, 1.0, 1.0, 1.0);
                        let color = Some(color);
                        let value:VShaderOut = VShaderOut {
                        }

                        let screenPos = float4::new(x, y, 0, 0);
                        let fin_vec = FShaderIn {
                            screenX: x,
                            screenY: y,
                            depth: 0.0, //TODO
                            value:
                        };
                        output.push(fin_vec)
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
