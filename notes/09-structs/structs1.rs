struct Point(f64, f64);

fn main() {
    let p1 = Point(4.0, 5.0);
    println!("Point #1: ({}, {})", p1.0, p1.1);
}