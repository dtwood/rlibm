#![allow(unused_parens)]
use super::utils::{Transmute};

macro_rules! p {
    ($x: expr) => { {
        let x = $x;
        println!("{:?}", $x);
        x
    } }
}

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
const TINY: f64 = 1.0e-300;

pub fn sqrt(x: f64) -> f64 {
    if x.is_nan() {
        return x*x+x;
    }

    if x.is_infinite() {
        if x.is_sign_positive() {
            return x;
        } else {
            return x*x+x; // sNaN
        }
    }

    if x == 0.0 {
        return x;
    }

    if x < 0.0 {
        return (x-x)/(x-x); // sNaN
    }


    let (mut ix0, mut ix1) = x.to_u32s();

    // double z;
    let sign: u32 = 0x80000000;
    // int32_t sign = (int)0x80000000;
    // int32_t ix0,s0,q,m,t,i;
    // u_int32_t r,t1,s1,ix1,q1;

    // EXTRACT_WORDS(ix0,ix1,x);

    let mut m: u32 = (ix0>>20);
    if(m==0) {    //     //     //     // /* subnormal x */
        while(ix0==0) {
            m -= 21;
            ix0 |= (ix1>>11); ix1 <<= 21;
        }
        let mut i: u32 = 0;
        loop {
            ix0 <<= 1;
            i += 1;
            if ix0 & 0x00100000 != 0 {
                break;
            }
        }
        m -= i-1;
        ix0 |= (ix1>>(32-i));
        ix1 <<= i;
    }
    m -= 1023;    // /* unbias exponent */
    ix0 = (ix0&0x000fffff)|0x00100000;
    if(m&1 != 0){    // /* odd m, double x to make it even */
        ix0 += ix0 + ((ix1&sign)>>31);
        ix1 += ix1;
    }
    m >>= 1;    // /* m = [m/2] */

    /* generate sqrt(x) bit by bit */
    ix0 += ix0 + ((ix1&sign)>>31);
    ix1 += ix1;
    let mut s0: u32 = 0;
    let mut s1: u32 = 0;
    let mut q: u32 = 0;
    // q = q1 = s0 = s1 = 0;    // /* [q,q1] = sqrt(x) */
    let mut r = 0x00200000;    //     // /* r = moving bit from right to left */

    while(r != 0) {
        let t = s0+r;
        if(t<=ix0) {
            s0   = t+r;
            ix0 -= t;
            q   += r;
        }
        ix0 += ix0 + ((ix1&sign)>>31);
        ix1 += ix1;
        r>>=1;
    }

    r = sign;
    let mut t1: u32;
    let mut t;
    let mut q1 = 0;
    while(r != 0) {
        t1 = s1+r;
        t  = s0;
        if((t<ix0)||((t==ix0)&&(t1<=ix1))) {
            s1  = t1+r;
            if(((t1&sign)==sign)&&(s1&sign)==0) { s0 += 1 };
            ix0 -= t;
            if (ix1 < t1) { ix0 -= 1 };
            ix1 -= t1;
            q1  += r;
        }
        ix0 += ix0 + ((ix1&sign)>>31);
        ix1 += ix1;
        r>>=1;
    }

    /* use floating add to find out rounding direction */
    if((ix0|ix1)!=0) {
        let mut z = ONE-TINY; /* trigger inexact flag */
        if (z>=ONE) {
            z = ONE+TINY;
            if (q1==0xffffffff) { q1=0; q += 1;}
            else if (z>ONE) {
                if (q1==0xfffffffe) { q+=1 };
                q1+=2; 
            } else
            {  q1 += (q1&1);}
            }
    }
    ix0 = (q>>1)+0x3fe00000;
    ix1 =  q1>>1;
    if ((q&1)==1) { ix1 |= sign };
    ix0 += (m <<20);

    return f64::from_u32s((ix0, ix1));
}

#[cfg(test)]
mod tests {
    use super::sqrt;

    include!("../../unit_tests/sqrt.rsi");
}
