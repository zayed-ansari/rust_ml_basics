fn simple_linear_regression(x:&[f64],y:&[f64] ){
    let n = x.len();
    let x_sum:f64 = x.iter().sum();
    let y_sum:f64 = y.iter().sum();

    let x_mean = x_sum / n as f64;
    let y_mean = y_sum / n as f64;

  
    let numerator:f64 = x.iter().zip(y.iter())
    .map(|(x_i, y_i)| (x_i-x_mean) * (y_i - y_mean)).sum();

    let denominator: f64 = x.iter().map(|x_i: &f64| (x_i - x_mean).powi(2)).sum();

    let b1:f64 = numerator/denominator;
    let b0:f64 = y_mean - (b1 * x_mean);
    println!("{}, {}", b0, b1);

    let prediction: Vec<f64> = x.iter().map(|a| b0 + b1 * a).collect(); 

    println!("Prediction for x is {:?}", prediction);
}

fn main() {
    let x = [1.,2.,3.,4.,5.];
    let y = [4.2, 3.1,5.3,1.1,4.23];

    simple_linear_regression(&x, &y);
}
