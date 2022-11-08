#[derive(Debug)]
struct Fraction(i32, i32);

fn main() {
    let f1 = Fraction(2, 3);
    let f2 = Fraction(-1, 5);

    println!("{:?} x {:?} = {:?}", f1, f2, mult(&f1,&f2));
}

fn mult(f_1: &Fraction, f_2: &Fraction) -> Fraction {
    Fraction(f_1.0 * f_2.0, f_1.1 * f_2.1)
}