fn main(){
    let mut first_array = [80, 85, 90, 92, 99];
    println!("{:?}", first_array);
    double(&mut first_array);
    println!("{:?}", first_array);
}

fn double(a: &mut [i32]){
    for i in 0..a.len(){
        a[i] = 2*a[i];
    }
}