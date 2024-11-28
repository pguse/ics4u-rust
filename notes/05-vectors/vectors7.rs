// Merging different-sized vectors
// into a vector, alternating elements

fn main() {
    let m = vec![1,3,5,7,9,11,13];
    let n = vec![2,4,6,8];
    
    let q = merge(&m,&n);
    
    println!("Merged: {:?}",q);
}

fn merge(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    // initializing two values at a time
    let (mut i, mut j) = (0,0);
    // creating a new empty vector
    let mut c: Vec<i32> = Vec::new();
    loop {
        if i < a.len() {
            // add an element using the push() method
            c.push(a[i]);
            i += 1;
        }
        if j < b.len() {
            c.push(b[j]);
            j += 1;
        }
        if i == a.len() && j == b.len() {break}
    }
    c
}