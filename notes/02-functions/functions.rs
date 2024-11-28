fn main() {
    let length = 4;
    let width = 3;
    println!("Area: {}", area_rectangle(length,width));
}

fn area_rectangle(base: i32, height: i32) -> i32 {
    base * height
}