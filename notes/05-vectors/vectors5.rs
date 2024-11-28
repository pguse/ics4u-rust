// Merging same-sized arrays
// into a vector - alternating elements
fn main() {
    let a = [1,3,5,7];
    let b = [2,4,6,8];
    
    let mut c: Vec<i32> = Vec::new();
    
    for i in 0..a.len() {
        c.push(a[i]);
        c.push(b[i]);
    }

    println!("{:?}",c);
}