fn main(){
    let my_array = [2, 4, 6, 8];// modify this line
    println!("{:?}", my_array);
    double(my_array);           // modify this line
    println!("{:?}", my_array);
}

fn double(mut a: [i32;4]){  // modify this line
    for i in 0..a.len(){
        a[i] = 2*a[i];
    }
    // Add code here
}