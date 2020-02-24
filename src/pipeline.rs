#![allow(dead_code)]

use crate::types::*;
use crate::utils::color::*;
use crate::utils::triangle2d::*;
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

            //TODO: transform clip_pos to NDC

            //transform NDC to screenPos
            let mut ndc_pos = vout_mapped.clip_pos;
            if ndc_pos.x > 1.0 || ndc_pos.x < -1.0 || ndc_pos.y > 1.0 || ndc_pos.y < -1.0 {
                panic!("invalid ndc pos")
            }

            ndc_pos.x += 1.0;
            ndc_pos.y += 1.0;
            ndc_pos.x *= (width - 1) as f64 / 2.0;
            ndc_pos.y *= (height - 1) as f64 / 2.0;

            vout_mapped.screen_pos = Some(ndc_pos);
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
            let v0 = vout_vec[i0].screen_pos.unwrap();
            let v1 = vout_vec[i1].screen_pos.unwrap();
            let v2 = vout_vec[i2].screen_pos.unwrap();

            let tri2d = Triangle2d::new(&[v0.x, v0.y, v1.x, v1.y, v2.x, v2.y]);
            tri2d.get_pixels()
        })
        .flatten()
        .map(|(screen_x, screen_y)| FShaderIn {
            screen_x,
            screen_y,
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
        fb.set_pixel(fout.screen_x, fout.screen_y, color)
    })
}

pub fn process_pipeline<VS, FS>(
    vin_vec: &[VShaderIn],
    indices: &[usize],
    vs: VS,
    fs: FS,
    fb: &mut Framebuffer,
) where
    VS: Fn(&VShaderIn) -> VShaderOut + Send + Sync,
    FS: Fn(&FShaderIn) -> FShaderOut + Send + Sync,
{
    let vout_vec = process_vertices(&vin_vec, vs);
    let vout_vec_clipped = perform_clipping(&vout_vec);
    let vout_vec_mapped =
        perform_screen_mapping(&vout_vec_clipped, fb.get_width(), fb.get_height());
    let fin_vec = setup_triangle(&vout_vec_mapped, &indices);
    let fout_vec = process_fragments(&fin_vec, fs);
    merge_output(&fout_vec, fb);
}
