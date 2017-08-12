extern crate robust_sum;
extern crate robust_scale;

use robust_sum::robust_sum as rsum;
use robust_scale::robust_scale as rscale;

///Robust Product
pub fn product(a: &[f64], b: &[f64]) -> Vec<f64> {
    if (a.len()) == 1 {
        return rscale(b, a[0]);
    }
    if b.len() == 1 {
        return rscale(a, b[0]);
    }
    if a.len() == 0 || b.len() == 0 {
        return vec!(0f64);
    }
    let mut r = vec!(0f64);
    if a.len() < b.len() {
        for i in 0..a.len() {
            r = rsum(&r, &rscale(&b, a[i]))
        }
    } else {
        for i in 0..b.len() {
            r = rsum(&r, &rscale(&a, b[i]))
        }
    }
    return r;
}


#[cfg(test)]
mod robust_prod {
    #[test]
    fn rproduct() {}
}