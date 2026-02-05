use crate::prelude::Parity;
use std::collections::HashMap;

pub fn mean_unchecked(xs: &[f64]) -> f64 {
    return xs.iter().sum::<f64>() / (xs.len() as f64);
}

pub fn median_unchecked(xs: &[f64]) -> f64 {
    let mut v = xs.to_vec();
    v.sort_by(f64::total_cmp);

    let v_len = v.len();
    let mid = v_len / 2;

    if v_len.is_even() {
        (v[mid - 1] + v[mid]) / 2.0
    } else {
        v[mid]
    }


} 

pub fn mode_unchecked(xs: &[i64]) -> i64 {

    let frequencies = xs
          .iter()
          .copied()
          .fold(HashMap::new(), |mut map, val|{
              map.entry(val)
                 .and_modify(|frq|*frq+=1)
                 .or_insert(1);
              map
          });
    println!("{:?}", frequencies);
    let mode = frequencies.iter().max_by_key(|(_,c)|*c).unwrap().0;
    
    return *mode;
    
}