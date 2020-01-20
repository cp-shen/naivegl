use crate::types::*;
use itertools::Itertools;
use rayon::prelude::*;

pub fn edge_fn<T>(a: (T, T), b: (T, T), p: (T, T)) -> T
where
    T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Copy,
{
    // the 3 vertices should be counter clockwise
    return (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
}

pub struct Triangle2d {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

impl Triangle2d {
    /// Test if a point is in the triangle
    /// Top-left rule is applied
    pub fn overlaps_point(&self, x: f64, y: f64) -> bool {
        let mut overlaps = true;

        let w0 = edge_fn((self.x1, self.y1), (self.x2, self.y2), (x, y));
        let w1 = edge_fn((self.x2, self.y2), (self.x0, self.y0), (x, y));
        let w2 = edge_fn((self.x0, self.y0), (self.x1, self.y1), (x, y));

        let edge01 = (self.x1 - self.x0, self.y1 - self.y0);
        let edge02 = (self.x2 - self.x0, self.y2 - self.y0);
        let edge12 = (self.x2 - self.x1, self.y2 - self.y1);

        overlaps &= if w0 == 0.0 {
            (edge12.1 == 0.0 && edge12.0 < 0.0) || edge12.1 < 0.0
        } else {
            w0 > 0.0
        };
        overlaps &= if w1 == 0.0 {
            (edge02.1 == 0.0 && edge02.0 < 0.0) || edge02.1 < 0.0
        } else {
            w1 > 0.0
        };
        overlaps &= if w2 == 0.0 {
            (edge01.1 == 0.0 && edge01.0 < 0.0) || edge01.1 < 0.0
        } else {
            w2 > 0.0
        };

        overlaps
    }

    pub fn get_pixels(&self) -> Vec<(usize, usize)> {
        if self.x0 < 0.0
            || self.y0 < 0.0
            || self.x1 < 0.0
            || self.y1 < 0.0
            || self.x2 < 0.0
            || self.y2 < 0.0
        {
            panic!(
                "to get all pixels,
                coords of the triangle vertices should not be negative"
            );
        }

        //Compute triangle bounding box
        let min_x: usize = self.x0.min(self.x1).min(self.x2).floor() as usize;
        let min_y: usize = self.x0.min(self.x1).min(self.y2).floor() as usize;
        let max_x: usize = self.y0.max(self.y1).max(self.y2).floor() as usize;
        let max_y: usize = self.y0.max(self.y1).max(self.y2).floor() as usize;

        let candidates: Vec<(usize, usize)> =
            (min_x..=max_x).cartesian_product(min_y..=max_y).collect();
        candidates
            .par_iter()
            .filter_map(|point: &(usize, usize)| {
                if self.overlaps_point(point.0 as f64, point.1 as f64) {
                    Some(point.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[allow(dead_code)]
pub struct Camera {
    near: f64,
    far: f64,
    fovy: f64,
    aspect: f64,
}

#[allow(dead_code)]
pub struct Transform {
    position: float3,
    scale: float3,
    rotation: cgmath::Quaternion<f64>,
}
