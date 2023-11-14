fn main() {
    // what is different from an array?
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(10);
    v.push(15);
    // what is the same as an array?
    // indexing
    println!("{}", v[1]);
    // output for debugging purposes
    println!("{:?}", v);
    // mutating elements
    v[1] = 100;
    // looping through the vector
    for i in 0..v.len() {
        print!("{} ", v[i])
    }
}