fn main() {
    // how to combine two vectors?
    // method #1
    let mut v = vec![5, 10, 15];
    let mut w = vec![20, 25, 30];
    v.append(&mut w);
    // notice: w is emptied, but still exists
    println!("{:?}", v);
    println!("{:?}", w);
}