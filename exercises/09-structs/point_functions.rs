#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
struct Point(f64, f64);


fn main() {
    let p1 = Point(4.0, 5.0);
    let p2 = Point(-1.0, -7.0);
    println!("{:?}", p1);
    println!("Slope: {}", slope(&p1,&p2));
    println!("Distance: {}", distance(&p1,&p2));
}

fn distance(p1: &Point, p2: &Point) -> f64 {
	0.0
}

fn slope(p1: &Point, p2: &Point) -> f64 {
	0.0
}