use itertools::Itertools;
use rayon::prelude::*;

/// the triangle is facing front if the 3 vertices are not counter-clockwise
pub struct Triangle2d {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

impl Triangle2d {
    pub fn new(vertices: &[f64]) -> Triangle2d {
        if vertices.len() < 6 {
            panic!("provided data is not enough to build triangle2d");
        };

        let tri = Triangle2d {
            x0: vertices[0],
            y0: vertices[1],
            x1: vertices[2],
            y1: vertices[3],
            x2: vertices[4],
            y2: vertices[5],
        };

        //if tri.is_colinear() {
        //panic!("the 3 vertices of a triangle should not be colinear");
        //};

        tri
    }

    fn is_counter_clockwise(&self) -> bool {
        let val =
            (self.y1 - self.y0) * (self.x2 - self.x1) - (self.x1 - self.x0) * (self.y2 - self.y1);
        val < 0.0
    }

    fn is_colinear(&self) -> bool {
        let val =
            (self.y1 - self.y0) * (self.x2 - self.x1) - (self.x1 - self.x0) * (self.y2 - self.y1);
        val == 0.0
    }

    /// test if a point is in the triangle
    /// top-left rule is applied
    fn overlaps_point(&self, x: f64, y: f64) -> (bool, f64, f64, f64) {
        if !self.is_counter_clockwise() {
            return (false, 0.0, 0.0, 0.0);
        }

        let mut overlaps = true;

        let edge01 = (self.x1 - self.x0, self.y1 - self.y0);
        let edge02 = (self.x2 - self.x0, self.y2 - self.y0);
        let edge12 = (self.x2 - self.x1, self.y2 - self.y1);

        let (alpha, beta, gamma) = self.get_barycentric_coord(x, y);

        overlaps &= if alpha == 0.0 {
            (edge12.1 == 0.0 && edge12.0 < 0.0) || edge12.1 < 0.0
        } else {
            alpha > 0.0
        };
        overlaps &= if beta == 0.0 {
            (edge02.1 == 0.0 && edge02.0 < 0.0) || edge02.1 < 0.0
        } else {
            beta > 0.0
        };
        overlaps &= if gamma == 0.0 {
            (edge01.1 == 0.0 && edge01.0 < 0.0) || edge01.1 < 0.0
        } else {
            gamma > 0.0
        };

        overlaps &= alpha > 0.0;
        overlaps &= beta > 0.0;
        overlaps &= gamma > 0.0;

        (overlaps, alpha, beta, gamma)
    }

    pub fn get_pixels(&self) -> Vec<(usize, usize, f64, f64, f64)> {
        #[cfg(debug_assertions)]
        println!(
            "x0:{}, y0:{}, x1:{}, y1:{}, x2:{}, y2:{}",
            self.x0, self.y0, self.x1, self.y1, self.x2, self.y2,
        );

        if !self.is_counter_clockwise() {
            return vec![];
        }

        if self.x0 < 0.0
            || self.y0 < 0.0
            || self.x1 < 0.0
            || self.y1 < 0.0
            || self.x2 < 0.0
            || self.y2 < 0.0
        {
            panic!("to get all pixels, coords of the triangle vertices should not be negative");
        }

        //Compute triangle bounding box
        let min_x: usize = self.x0.min(self.x1).min(self.x2).floor() as usize;
        let min_y: usize = self.y0.min(self.y1).min(self.y2).floor() as usize;
        let max_x: usize = self.x0.max(self.x1).max(self.x2).floor() as usize;
        let max_y: usize = self.y0.max(self.y1).max(self.y2).floor() as usize;

        let candidates: Vec<(usize, usize)> =
            (min_x..=max_x).cartesian_product(min_y..=max_y).collect();

        candidates
            .par_iter()
            .filter_map(|point: &(usize, usize)| {
                let (overlaps, alpha, beta, gamma) =
                    self.overlaps_point(point.0 as f64, point.1 as f64);
                if overlaps {
                    Some((point.0, point.1, alpha, beta, gamma))
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_barycentric_coord(&self, x: f64, y: f64) -> (f64, f64, f64) {
        let xa = self.x0;
        let xb = self.x1;
        let xc = self.x2;

        let ya = self.y0;
        let yb = self.y1;
        let yc = self.y2;

        let mut gamma = (ya - yb) * x + (xb - xa) * y + xa * yb - xb * ya;
        gamma /= (ya - yb) * xc + (xb - xa) * yc + xa * yb - xb * ya;

        let mut beta = (ya - yc) * x + (xc - xa) * y + xa * yc - xc * ya;
        beta /= (ya - yc) * xb + (xc - xa) * yb + xa * yc - xc * ya;

        let alpha = 1.0 - beta - gamma;

        (alpha, beta, gamma)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_triangle2d() {
        //TODO
    }
}
