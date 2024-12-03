# Files

Reading text files in Rust is straightforward with the `std::fs` module. Here's a basic example to get you started:

1. **Import the necessary modules**:
```rust
use std::fs::File;
use std::io::prelude::*;
```

2. **Open the file and read its contents**:
```rust
fn read_file(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents

}

fn main() {
    let file_path = "src/input.txt";
    let data = read_file(file_path);
    println!("{}", data);
}
```

In this example:
- `File::open(file_path) opens the file at the specified path` or returns an error if the file is not found.
- `file.read_to_string(&mut contents) reads the file's contents into a String` or returns an error if the file cannot be read.

This should help you get started with reading text files in Rust. If you have any specific requirements or run into issues, feel free to ask!