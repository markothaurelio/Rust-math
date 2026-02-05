use basic_math::stats;
//use basic_math::linear;

fn main() {
//    let v = vec![1.0, 2.0, 3.0];

//    let v_r = vec![1.0, 2.0, 3.0,5.0,10.0,7.0];
//    let v_e = vec![];
    
//    let v_i = vec![1,2,3,4];

//    let v_m = vec![1, 2, 2, 3, 3, 3, 4];

    let v_iqr = vec![2.0, 4.0, 5.0, 7.0, 8.0, 10.0, 12.0, 15.0];


//    println!("mean = {}", stats::mean_unchecked(&v_e));
//    println!("median = {}", stats::median_unchecked(&v));
//    println!("mode = {}", stats::mode_unchecked(&v_m));

//    println!("range = {}", stats::range_unchecked(&v_r));
    println!("slice = {:?}", v_iqr);
    println!("IQR = {}", stats::iqr_unchecked(&v_iqr));

//    println!("dot  = {}", linear::dot(&v, &v));


}
