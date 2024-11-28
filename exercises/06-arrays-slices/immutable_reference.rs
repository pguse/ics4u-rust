fn main(){
    let first_array = [85, 90, 92, 80, 99];
    let second_array = [300, 100, 200];

    // Slice of an array - index 1 up to 4
    println!("{:?}", &first_array[1..4]);

    // The minimum() function can be called
    // on an array of any length, since
    // the argument is an array slice (a 
    // referenceto an array)
    println!("{:?}", minimum(&first_array));
    println!("{:?}", minimum(&second_array));
}

// Note:  A entire array is not copied
// when the minimum() function is called.  Only
// a  reference to the array is passed into
// (borrowed by)  the function.
fn minimum(a: &[i32]) -> i32 {
    let mut min = a[0];
    for i in 0..a.len() {
        if a[i] < min {
            min = a[i];
        }
    }
    min
}