struct Student {
    name: String,
    grade: u8,
}

fn main() {
    let s1 = Student {
        name: String::from("Guido van Rossum"),
        grade: 11,
    };

    println!("{} is in grade {}.", s1.name, s1.grade);
}