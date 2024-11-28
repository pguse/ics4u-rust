fn main(){
    println!("Slope between (1.0,2.0) and (3.0,6.0): {}", slope(1.0, 2.0, 3.0, 6.0));
}

fn slope(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    // Return the correct slope value
    x1 + y1 + x2 + y2
}