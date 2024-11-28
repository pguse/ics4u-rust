#[derive(Debug)]
struct Point(f64, f64);

fn main() {
    let p1 = Point(4.0, 5.0);
    let p2 = Point(1.0, 5.0);
    println!("Midpoint: {:?}", midpoint(p1, p2));
}

fn midpoint(p1: Point, p2: Point) -> Point {
	Point((p1.0 + p2.0) / 2.0, (p1.1 + p2.1) / 2.0)
}