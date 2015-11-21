use super::utils::{Transmute};

macro_rules! assert_approx_eq {
    ($left: expr, $right: expr) => { {
        let left = $left;
        let right = $right;
        if !(left * 0.999 <= right) {
            panic!("assertion failed: `(left ≈≈ right)` (left: `{:?}`, right: `{:?}`)",
                left, right);
        }
        if !(right * 0.999 <= left) {
            panic!("assertion failed: `(left ≈≈ right)` (left: `{:?}`, right: `{:?}`)",
                left, right);
        }
    } }
}

const ONE: f64 = 1.0;
const HALF: [f64; 2] = [0.5,-0.5];
const HUGE: f64 = 1.0e+300;
const O_THRESHOLD: f64 = 7.09782712893383973096e+02; // 0x40862E42, 0xFEFA39EF
const U_THRESHOLD: f64 = -7.45133219101941108420e+02; // 0xc0874910, 0xD52D3051
const LN2HI: [f64; 2] = [
    6.93147180369123816490e-01, // 0x3fe62e42, 0xfee00000
    -6.93147180369123816490e-01, // 0xbfe62e42, 0xfee00000
];
const LN2LO: [f64; 2] = [
    1.90821492927058770002e-10, // 0x3dea39ef, 0x35793c76
    -1.90821492927058770002e-10, // 0xbdea39ef, 0x35793c76
];
const INVLN2: f64 = 1.44269504088896338700e+00; // 0x3ff71547, 0x652b82fe
const POLY1: f64 = 1.66666666666666019037e-01; // 0x3FC55555, 0x5555553E
const POLY2: f64 = -2.77777777770155933842e-03; // 0xBF66C16C, 0x16BEBD93
const POLY3: f64 = 6.61375632143793436117e-05; // 0x3F11566A, 0xAF25DE2C
const POLY4: f64 = -1.65339022054652515390e-06; // 0xBEBBBD41, 0xC5D26BF1
const POLY5: f64 = 4.13813679705723846039e-08; // 0x3E663769; 0x72BEA4D0

const TWOM1000: f64 = 9.33263618503218878990e-302; // 2**-1000=0x01700000,0
const TWOP1023: f64 = 89884656743115795386465259539451236680898848947115328636715040578866337902750481566354238661203768010560056939935696678829394884407208311246423715319737062188883946712432742638151109800623047059726541476042502884419075341171231440736956555270413618581675255342293149119973622969239858152417678164812112068608.000000;


pub fn exp(x: f64) -> f64 {
    let y: f64;
    let hi: f64;
    let lo: f64;
    let c: f64;

    let k: i32;

    let xsb = (x.high_u32() >> 31) == 1;
    let hx = x.high_u32() & 0x7fffffff;

    let mut x = x;

    if hx >= 0x40862E42 { // |x| >= 709.78...
        if hx >= 0x7ff00000 {
            let lx: u32 = x.low_u32();
            if ((hx & 0x000fffff) | lx) != 0 {
                return x + x; // NaN
            } else {
                return if xsb { 0.0 } else { x }; // exp(+-inf) = (inf, 0)
            }
        }

        if x > O_THRESHOLD {
            return HUGE * HUGE;
        }
        if x < U_THRESHOLD {
            return TWOM1000 * TWOM1000;
        }
    }

    if x == 1.0 {
        return 2.718281828459045235360;
    }

    if hx > 0x3fd62e42 { // |x| > 0.5 ln2
        if hx < 0x3ff0a2b2 { // |x| < 1.5 ln2
            hi = x - LN2HI[xsb as usize];
            lo = LN2LO[xsb as usize];
            k = if xsb { -1 } else { 1 };
        } else {
            k = (INVLN2 * x + HALF[xsb as usize]) as i32;
            let t = k as f64;
            hi = x - t * LN2HI[0];
            lo = t * LN2LO[0];
        }
        x = hi - lo;
    } else if hx < 0x3e300000 {
        if HUGE + x > ONE {
            return ONE + x;
        }
        k = 0xDEADBEEF;
        hi = 0xDEADBEEF as f64;
        lo = 0xDEADBEEF as f64;
    } else {
        k = 0;
        hi = 0xDEADBEEF as f64;
        lo = 0xDEADBEEF as f64;
    }

    let t = x * x;
    let twopk = f64::from_u32s(
            (0x3ff00000 + ((if k >= -1021 { k } else { k + 1000 } as u32) << 20),
            0 as u32)
        );

    c = x - t * (POLY1 + t * (POLY2 * t * (POLY3 * t * (POLY4 * t * POLY5))));

    if k == 0 {
        return ONE - ((x * c) / (c - 2.0) - x);
    } else {
        y = ONE - ((lo - (x * c) / (2.0 - c)) - hi);
    }

    if k >= -1021 {
        if k == 1024 {
            return y * 2.0 * TWOP1023;
        }
        return y * twopk;
    } else {
        return y * twopk * TWOM1000;
    }
}

#[cfg(test)]
mod tests {
    use std::f64;

    const E_APPROX: f64 = 2.718281828459045;

    #[test]
    fn test_zero() {
        assert_approx_eq!(super::exp(0.0), 1.0);
    }

    #[test]
    fn test_one() {
        assert_approx_eq!(super::exp(1.0), E_APPROX);
    }

    #[test]
    fn test_larger() {
        for i in 0..10 {
            println!("{}", i);
            assert_approx_eq!(super::exp(i as f64), E_APPROX.powi(i));
        }
    }

    #[test]
    fn test_f64_max() {
        assert_approx_eq!(super::exp(f64::MAX), f64::INFINITY);
    }

    #[test]
    fn test_f64_min() {
        assert_eq!(super::exp(f64::MIN), 0.0);
    }

    #[test]
    fn test_nan() {
        assert!(super::exp(f64::NAN).is_nan());
    }

    #[test]
    fn test_plus_inf() {
        assert_approx_eq!(super::exp(f64::INFINITY), f64::INFINITY);
    }

    #[test]
    fn test_neg_inf() {
        assert_approx_eq!(super::exp(-f64::INFINITY), 0.0);
    }
}
