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
            let mut ndc_pos = vout_mapped.clipPos;
            if ndc_pos.x > 1.0 || ndc_pos.x < -1.0 || ndc_pos.y > 1.0 || ndc_pos.y < -1.0 {
                panic!("invalid ndc pos")
            }

            ndc_pos.x += 1.0;
            ndc_pos.y += 1.0;
            ndc_pos.x *= (width - 1) as f64 / 2.0;
            ndc_pos.y *= (height - 1) as f64 / 2.0;

            vout_mapped.screenPos = Some(ndc_pos);
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

            let tri2d = Triangle2d {
                x0: v0.x,
                y0: v0.y,
                x1: v1.x,
                y1: v1.y,
                x2: v2.x,
                y2: v2.y,
            };

            tri2d.get_pixels()
        })
        .flatten()
        .map(|(screenX, screenY)| FShaderIn {
            screenX,
            screenY,
            depth: 0.0,
            value: vout_vec[0].clone(), //FIXME
        })
        .collect()
}

pub fn process_fragments<FS>(fin_vec: &[FShaderIn], fs: FS) -> Vec<FShaderOut>
where
    FS: Fn(&FShaderIn) -> FShaderOut + Send + Sync,
{
    fin_vec.par_iter().map(fs).collect()
}

pub fn merge_output(fout_vec: &[FShaderOut], fb: &mut Framebuffer) {
    fout_vec.iter().for_each(|fout| {
        let c = fout.color;
        let color = (
            get_8bit_color(c.x),
            get_8bit_color(c.y),
            get_8bit_color(c.z),
            get_8bit_color(c.w),
        );
        fb.set_pixel(fout.screenX, fout.screenY, color)
    })
}

fn get_8bit_color(f: f64) -> u8 {
    assert_eq!(f >= 0.0, true);
    assert_eq!(f <= 1.0, true);
    (f * 255.0).round() as u8
}
