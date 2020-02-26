#![allow(dead_code)]

use crate::framebuffer::*;
use crate::shader_common::*;
use crate::utils::*;
use cgmath::prelude::*;
use rayon::prelude::*;
use std::marker::{Send, Sync};

pub fn process_vertices<VS>(vin_vec: &[VShaderIn], vs: VS) -> Vec<VShaderOut>
where
    VS: Fn(&VShaderIn) -> VShaderOut + Send + Sync,
{
    vin_vec.par_iter().map(vs).collect()
}

pub fn perform_clipping(vout_vec: &[VShaderOut]) -> Vec<Option<VShaderOut>> {
    vout_vec
        .par_iter()
        .map(|vout: &VShaderOut| {
            let clip_pos = vout.clip_pos;
            assert_eq!(
                clip_pos.w > 0.0,
                true,
                "w component of clip_pos is not greater than zero"
            );

            let mut survive = true;

            let w = clip_pos.w;
            survive &= clip_pos.x >= -w || clip_pos.x <= w;
            survive &= clip_pos.y >= -w || clip_pos.y <= w;
            survive &= clip_pos.z >= -w || clip_pos.z <= w;

            if survive {
                Some(vout.to_owned())
            } else {
                None
            }
        })
        .collect()
}

pub fn perform_screen_mapping(
    vout_vec: &[Option<VShaderOut>],
    width: usize,
    height: usize,
) -> Vec<Option<VShaderOut>> {
    vout_vec
        .par_iter()
        .map(|vout_op: &Option<VShaderOut>| {
            match vout_op {
                None => None,
                Some(vout) => {
                    let mut vout_mapped = vout.to_owned();
                    let clip_pos = vout_mapped.clip_pos;

                    //transform clip coords to NDC
                    assert_eq!(
                        clip_pos.w > 0.0,
                        true,
                        "w component of clip_pos is not greater than zero"
                    );
                    let ndc_pos = float3 {
                        x: clip_pos.x / clip_pos.w,
                        y: clip_pos.y / clip_pos.w,
                        z: clip_pos.z / clip_pos.w,
                    };
                    //validate NDC
                    if ndc_pos.x > 1.0 || ndc_pos.x < -1.0 || ndc_pos.y > 1.0 || ndc_pos.y < -1.0 {
                        panic!("invalid ndc pos: x={}, y={}", ndc_pos.x, ndc_pos.y)
                    }

                    //transform NDC to screenPos
                    let screen_pos = float3 {
                        x: (ndc_pos.x + 1.0) * 0.5 * (width as f64 - 1.0),
                        y: (ndc_pos.y + 1.0) * 0.5 * (height as f64 - 1.0),
                        z: ndc_pos.z,
                    };

                    vout_mapped.screen_pos = Some(screen_pos);
                    Some(vout_mapped)
                }
            }
        })
        .collect()
}

pub fn setup_triangle(vout_vec: &[Option<VShaderOut>], indices: &[usize]) -> Vec<FShaderIn> {
    assert_eq!(
        indices.len() % 3,
        0,
        "length of indices should be dividable by 3"
    );

    indices
        .par_iter()
        .enumerate()
        .filter_map(|(i, _)| -> Option<(usize, usize, usize)> {
            if i % 3 == 0 {
                Some((indices[i], indices[i + 1], indices[i + 2]))
            } else {
                None
            }
        })
        .filter_map(|(i0, i1, i2)| -> Option<(usize, usize, usize)> {
            let mut survive = true;
            survive &= vout_vec[i0].is_some();
            survive &= vout_vec[i1].is_some();
            survive &= vout_vec[i2].is_some();
            if survive {
                Some((i0, i1, i2))
            } else {
                None
            }
        })
        .map(|(i0, i1, i2)| -> Vec<FShaderIn> {
            let v0 = vout_vec[i0].to_owned().unwrap();
            let v1 = vout_vec[i1].to_owned().unwrap();
            let v2 = vout_vec[i2].to_owned().unwrap();

            let scr0 = v0.screen_pos.unwrap();
            let scr1 = v1.screen_pos.unwrap();
            let scr2 = v2.screen_pos.unwrap();

            let tri2d = Triangle2d::new(&[scr0.x, scr0.y, scr1.x, scr1.y, scr2.x, scr2.y]);
            let pixels = tri2d.get_pixels();

            pixels
                .par_iter()
                .map(|(x, y, alpha, beta, gamma)| {
                    let screen_x = x.to_owned();
                    let screen_y = y.to_owned();
                    let depth = scr0.z * alpha + scr1.z * beta + scr2.z * gamma;

                    let clip_pos =
                        v0.clip_pos * *alpha + v1.clip_pos * *beta + v2.clip_pos * *gamma;

                    let screen_pos = if v0
                        .screen_pos
                        .and(v1.screen_pos.and(v2.screen_pos))
                        .is_some()
                    {
                        Some(
                            v0.screen_pos.unwrap() * *alpha
                                + v1.screen_pos.unwrap() * *beta
                                + v2.screen_pos.unwrap() * *gamma,
                        )
                    } else {
                        None
                    };

                    let mut world_normal = if v0
                        .world_normal
                        .and(v1.world_normal.and(v2.world_normal))
                        .is_some()
                    {
                        Some(
                            v0.world_normal.unwrap() * *alpha
                                + v1.world_normal.unwrap() * *beta
                                + v2.world_normal.unwrap() * *gamma,
                        )
                    } else {
                        None
                    };

                    let vert_color = if v0
                        .vert_color
                        .and(v1.vert_color.and(v2.vert_color))
                        .is_some()
                    {
                        Some(
                            v0.vert_color.unwrap() * *alpha
                                + v1.vert_color.unwrap() * *beta
                                + v2.vert_color.unwrap() * *gamma,
                        )
                    } else {
                        None
                    };

                    let world_pos = if v0.world_pos.and(v1.world_pos.and(v2.world_pos)).is_some() {
                        Some(
                            v0.world_pos.unwrap() * *alpha
                                + v1.world_pos.unwrap() * *beta
                                + v2.world_pos.unwrap() * *gamma,
                        )
                    } else {
                        None
                    };

                    if world_pos.is_some() && world_normal.is_none() {
                        let edge01 = v1.world_pos.unwrap() - v0.world_pos.unwrap();
                        let edge12 = v2.world_pos.unwrap() - v1.world_pos.unwrap();
                        let normal = edge01.truncate().cross(edge12.truncate()).normalize();
                        world_normal = Some(cgmath::vec4(normal.x, normal.y, normal.z, 0.0));
                    };

                    let interpolated = VShaderOut {
                        world_normal,
                        clip_pos,
                        screen_pos,
                        world_pos,
                        vert_color,
                    };

                    FShaderIn {
                        screen_x,
                        screen_y,
                        depth,
                        value: interpolated,
                    }
                })
                .collect()
        })
        .flatten()
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
