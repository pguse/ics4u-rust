#![allow(unused_variables)]

#[derive(Debug)]
struct Fraction(i32, i32);

impl Fraction {
    fn div(&self, f: &Fraction) -> Fraction {
        Fraction(1,1)
    }

    fn add(&self, f: &Fraction) -> Fraction {
        Fraction(1,1)
    }
    fn subt(&self, f: &Fraction) -> Fraction {
        Fraction(1,1)
    }
}

fn main() {
    let f1 = Fraction(2, 3);
    let f2 = Fraction(-1, 5);

    println!("{:?}", &f1.div(&f2));
    println!("{:?}", &f1.add(&f2));
    println!("{:?}", &f1.subt(&f2));
}