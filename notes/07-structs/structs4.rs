#[derive(Debug)]
struct Fraction(i32, i32);

impl Fraction {
    fn mult(self, f: &Fraction) -> Fraction {
        Fraction(self.0 * f.0, self.1 * f.1)
    }
}

fn main() {
    let f1 = Fraction(2, 3);
    let f2 = Fraction(-1, 5);

    println!("{:?}", f1.mult(&f2));
}