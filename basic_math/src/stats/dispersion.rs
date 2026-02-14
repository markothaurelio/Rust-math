
use crate::stats::{mean_unchecked, median_unchecked};

pub fn range_unchecked(xs: &[f64]) -> f64 {
    let min = xs.iter().copied().fold(f64::INFINITY,  |acc, x| f64::min(acc, x));
    let max = xs.iter().copied().fold(f64::NEG_INFINITY,  |acc, x| f64::max(acc, x));

    max - min
}

pub fn iqr_unchecked(xs: &[f64]) -> f64 {
    let mut v = xs.to_vec();
    v.sort_by(f64::total_cmp);

    let mid = v.len() / 2;
    println!("mid  = {}", mid);

    let (lower, upper) = if v.len() % 2 == 0 {
        (&v[..mid], &v[mid..])
    } else {
        (&v[..mid], &v[mid + 1..]) // skip the median
    };

    let q1 = median_unchecked(&lower);
    let q3 = median_unchecked(&upper);

    q3-q1

}

pub fn sample_variance_unchecked(xs: &[f64]) -> f64 {

    let mean = mean_unchecked(xs);

    let n = xs.len();

    let sample_variance = xs.iter().map(|&x| (x - mean).powf(2.0)).sum::<f64>() / (n - 1) as f64;

    sample_variance

}

pub fn sample_standard_deviation_unchecked(xs: &[f64]) -> f64 {
    sample_variance_unchecked(xs).sqrt()
}

pub fn z_scores_unchecked(xs: &[f64]) -> Vec<f64> {
    let mean = mean_unchecked(xs);
    let std = sample_standard_deviation_unchecked(xs);

    xs.iter().map(|&x| (x - mean) / std).collect()
}

pub fn covariance_unchecked(xs: &[f64], ys: &[f64]) -> f64 {

    let n = xs.len();

    let mean_x = mean_unchecked(xs);
    let mean_y = mean_unchecked(ys);


    let covariance = xs.iter()
        .zip(ys.iter())
        .map(|(&x, &y)| (x - mean_x) * (y - mean_y))
        .sum::<f64>() / (n - 1) as f64;

    covariance
}

pub fn correlation_unchecked(xs: &[f64], ys: &[f64]) -> f64 {
    
    let cov = covariance_unchecked(xs, ys);

    let xs_std = sample_standard_deviation_unchecked(xs);
    let ys_std = sample_standard_deviation_unchecked(ys);

    let correlation = cov/(xs_std*ys_std);

    correlation

}
