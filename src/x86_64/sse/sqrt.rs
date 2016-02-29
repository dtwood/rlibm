#[allow(dead_code)]
pub fn sqrt(x: f64) -> f64 {
    let output: f64;
    unsafe {
        asm!(
        "sqrtsd $0, $1" :
        "=x" (output) /* output */ :
        "x" (x) /* input */ :
        /* clobbers */ :
        "intel" /* options */ :
        );    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::sqrt;

    include!("../../../unit_tests/sqrt.rsi");
}
