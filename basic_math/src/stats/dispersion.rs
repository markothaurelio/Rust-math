pub fn range_unchecked(xs: &[f64]) -> f64 {
    let min = xs.iter().copied().fold(f64::INFINITY,  |acc, x| f64::min(acc, x));
    let max = xs.iter().copied().fold(f64::NEG_INFINITY,  |acc, x| f64::max(acc, x));

    max - min
}

pub fn iqr_unchecked(xs: &[f64]) -> f64 {
    let mut v = xs.to_vec();
    v.sort_by(|a,b| a.partial_cmp(b).unwrap());

    let q1 = quantile_linear_unchecked(&v, 0.25);
    let q3 = quantile_linear_unchecked(&v, 0.75);

    q3-q1
}

fn quantile_linear_unchecked(sorted: &[f64], p: f64) -> f64 {
    let n = sorted.len();

    if n == 1 {
        return sorted[0];
    }

    let idx = p * (n as f64 - 1.0);
    let lo = idx as usize;              // floor
    let hi = (idx.ceil()) as usize;

    println!("idx = {}", idx);
    println!("low = {}", lo);
    println!("high = -{}", hi);

    if lo == hi {
        unsafe { *sorted.get_unchecked(lo) }
    } else {
        let w = idx - lo as f64;
        unsafe {
            let a = *sorted.get_unchecked(lo);
            let b = *sorted.get_unchecked(hi);
            a * (1.0 - w) + b * w
        }
    }
}