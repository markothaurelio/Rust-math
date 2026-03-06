use core::f64;

use std::collections::HashSet;


pub enum Average {
    Binary,
    Macro,
    Micro,
    Weighted,
}

pub fn accuracy(ys: &[i32], ys_h: &[i32]) -> f64 {

    assert!(ys.len() == ys_h.len());
    assert!(ys.len() > 0);

    let n = ys.len();
    
    let correct = ys.iter().zip(ys_h.iter()).filter(|(yt, yp)| yt == yp).count();

    correct as f64 / n as f64

}

pub fn precision(ys: &[i32], ys_h: &[i32], average: Average) -> f64 {
    assert!(ys.len() == ys_h.len());
    assert!(!ys.is_empty());

    match average {
        Average::Binary => precision_binary(ys, ys_h),
        Average::Macro => precision_macro(ys, ys_h),
        Average::Micro => panic!("Not implemented"),
        _ => panic!("Invalid average type"), // default case
    }

}


pub fn precision_macro(ys: &[i32], ys_h: &[i32]) -> f64 {
    // Collect unique classes from true labels
    let classes: HashSet<i32> = ys.iter().cloned().collect();

    let mut total_precision = 0.0;
    let mut class_count = 0;

    for &class in &classes {
        let mut tp = 0;
        let mut fp = 0;

        for (&yt, &yp) in ys.iter().zip(ys_h.iter()) {
            if yp == class {
                if yt == class {
                    tp += 1;
                } else {
                    fp += 1;
                }
            }
        }

        if tp + fp > 0 {
            total_precision += tp as f64 / (tp + fp) as f64;
            class_count += 1;
        }
    }

    total_precision / class_count as f64
}

fn precision_binary(ys: &[i32], ys_h: &[i32]) -> f64 {

    
    let tp = ys.iter().zip(ys_h.iter()).filter(|&(&yt, &yp)| yt == 1 && yp == 1).count();
    let fp = ys.iter().zip(ys_h.iter()).filter(|&(&yt, &yp)| yt == 0 && yp == 1).count();

    assert!((tp + fp) != 0);

    tp as f64 / (tp + fp) as f64

}

pub fn recall(ys: &[i32], ys_h: &[i32]) -> f64 {

    assert!(ys.len() == ys_h.len());
    
    let tp = ys.iter().zip(ys_h.iter()).filter(|&(&yt, &yp)| yt == 1 && yp == 1).count();
    let fneg = ys.iter().zip(ys_h.iter()).filter(|&(&yt, &yp)| yt == 1 && yp == 0).count();

    assert!((tp + fneg) != 0);

    tp as f64 / (tp + fneg) as f64

}

pub fn specificity(ys: &[i32], ys_h: &[i32]) -> f64 {

    assert!(ys.len() == ys_h.len());
    
    let tn = ys.iter().zip(ys_h.iter()).filter(|&(&yt, &yp)| yt == 0 && yp == 0).count();
    let fp = ys.iter().zip(ys_h.iter()).filter(|&(&yt, &yp)| yt == 0 && yp == 1).count();

    assert!((tn + fp) != 0);

    tn as f64 / (tn + fp) as f64

}

pub fn f1_score(ys: &[i32], ys_h: &[i32], average: Average) -> f64 {

    assert!(ys.len() == ys_h.len());

    let precision = precision(ys, ys_h, average);
    let recall  = recall(ys, ys_h);
    
    assert!(precision+recall != 0.0);

    2.0*((precision * recall)/(precision+recall))
}