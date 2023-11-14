fn main() {
    // how to combine two vectors?
    // method #3
    let mut v = vec![5, 10, 15];
    let w = vec![20, 25, 30];
    v.extend(&w);
    println!("{:?}", v);
    // w is unchanged
    println!("{:?}", w);
}