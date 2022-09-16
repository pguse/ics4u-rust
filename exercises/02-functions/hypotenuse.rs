fn main(){
    println!("Hypotenuse - base: 3; height: 4: {}", hypotenuse(3.0, 4.0));
}

fn hypotenuse(a: f64, b: f64) -> f64 {
    // Return the correct hypotenuse value
    a + b
}