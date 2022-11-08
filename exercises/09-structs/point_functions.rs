struct Point(f64, f64);

fn main() {
    let p1 = Point(4.0, 5.0);
    let p2 = Point(-1.0, -7.0);
    println!("Slope: {}", slope(&p1,&p2));
    println!("Distance: {}", distance(&p1,&p2));
}

fn distance(p_1: &Point, p_2: &Point) -> f64 {
	0.0
}

fn slope(p_1: &Point, p_2: &Point) -> f64 {
	0.0
}