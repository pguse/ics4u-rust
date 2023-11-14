fn main() {
    // how to combine a vector with an array?
    let mut v = vec![5, 10, 15];
    let a = [20, 25, 30];
    v.extend_from_slice(&a);
    println!("{:?}", v);
    // a is unchanged
    println!("{:?}", a);
}