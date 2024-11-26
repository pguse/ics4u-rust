fn main() {
    let marks = [80, 70, 90, 85, 75];
    
    println!("The average mark is: {}", average(marks));
    println!("All marks: {:?}", marks);
}

fn average(arr: [i32; 5]) -> i32 {
    let mut total = 0;

    for value in arr {
        total += value;
    }
    total / (arr.len() as i32)
}