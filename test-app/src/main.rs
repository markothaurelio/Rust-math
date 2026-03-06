use core::stats;

fn main() {

    


//    let v_iqr = vec![2.0, 4.0, 5.0, 7.0, 8.0, 10.0, 12.0, 15.0];

//    let xs = vec![2.0, 4.0, 5.0, 7.0, 8.0, 10.0, 12.0, 15.0];
//    let ys = vec![4.0, 8.0, 10.0, 14.0, 16.0, 20.0, 24.0, 30.0]; // ys = 2 * xs

//test
//    println!("mean = {}", stats::mean_unchecked(&v_e));
//    println!("median = {}", stats::median_unchecked(&v));
//    println!("mode = {}", stats::mode_unchecked(&v_m));

//    println!("range = {}", stats::range_unchecked(&v_r));
//    println!("slice = {:?}", v_iqr);
//    println!("IQR = {}", stats::iqr_unchecked(&v_iqr));

    let ys: Vec<i32> = vec![1, 0, 1, 1, 0, 1, 0, 0, 1, 1];
    let ys_h: Vec<i32> = vec![1, 1, 1, 0, 0, 1, 0, 0, 1, 0];


    //println!("Sample Variance = {}", stats::sample_variance_unchecked(&v_iqr));

    
    //println!("Z scores = {:?}", stats::z_scores_unchecked(&v_iqr));
    
    //println!("Covariance = {}", stats::covariance_unchecked(&xs, &ys));

    
    //println!("Correlation {}", stats::correlation_unchecked(&xs, &ys));
        
    println!("Accuracy {}", stats::accuracy(&ys, &ys_h));
    println!("Precision {}", stats::precision(&ys, &ys_h, stats::Average::Binary));
    println!("Recall {}", stats::recall(&ys, &ys_h));
    println!("specificity {}", stats::specificity(&ys, &ys_h));
    println!("F1 Score {}", stats::f1_score(&ys, &ys_h, stats::Average::Binary));
    

}
