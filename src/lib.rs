extern crate robust_sum;
extern crate robust_scale;

use robust_sum::robust_sum as rsum;
use robust_scale::robust_scale as rscale;

///Robust product
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
mod test_robust_prod {
    use super::product;

    #[test]
    fn test_product() {
        fn pow2(n: i32) -> f64 {
            return 2f64.powi(n);
        }

        for i in -20..(20 + 1) {
            for j in -20..(20 + 1) {
                let (fi, fj) = (i as f64, j as f64);
                assert_eq!(product(&vec!(fi), &vec!(fj)), [fi * fj]);
            }
        }

        assert_eq!(product(
            &vec!(pow2(-50), pow2(50)),
            &vec!(pow2(-50), pow2(50))
        ), [pow2(-100), pow2(1), pow2(100)]);
    }
}