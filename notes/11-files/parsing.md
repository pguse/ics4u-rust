# Parsing Strings

## split() method

We often need to take a **String** and extract information from it, whether it be sub-strings or numerical values. This is often done using the [`split()`](https://doc.rust-lang.org/std/string/struct.String.html) method that is part of the **String** type. Here is an [example](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ec2e4937f89e76ff06b1ab8952fcab70) ,

```rust
fn main() {
    let courses =  "English 85\nScience 80\nHistory 90\nGeography 75";
    print!("{}", courses);
}
```

where the output is
```
English 85
Science 80
History 90
```

The special character `\n` represents a **new-line** and is responsible for the output on three separate lines. If we want to extract each line from the string, we could use the `split()` method, providing it with the `\n` special character.  For example, the following [code](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1e22671ed4a8ccca8d331543c5cc42bd)

```rust
fn main() {
    let courses =  "English 85\nScience 80\nHistory 90\nGeography 75";
    let courses: Vec<&str> = courses.split("\n").collect();
    print!("{:?}", courses);
}
```

uses the `split()` method to produce an iterator, which then needs the `collect()` method to convert it into some sort of collection, such as a [vector](notes/05-vectors/vectors.md). Notice that `collect()` **requires** a **type annotation** to indicate that we want the `courses` variable to be of type `Vec<&str>`. The output for this program will be

```
["English 85", "Science 80", "History 90", "Geography 75"]
```

Now we may want to **parse** each String in the  [vector](notes/05-vectors/vectors.md).  For example, the following [code](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=67261cc12253ae60427a712202cc550e)

```rust
fn main() {
    let courses =  "English 85";
    let courses: Vec<&str> = courses.split(" ").collect();
    print!("{:?}", courses);
}
```

uses the `split()` method to separate a **String** according the a single whitespace character, producing the output,

```
["English", "85"]
```

## parse() method

The `parse()` method can be used in a couple different ways in Rust.  For example, in the following [code](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=290c91667997e22d712f889971d7681e),

```rust
fn main() {
    let data = "2024";
    let n: i32 = data.parse().unwrap();
    println!("{}", n);
}
```

produces the output

```
2024
```

In Rust, the `parse()` method is used to convert a string into another type. The type that `parse()` returns is determined by the type you specify. It returns a `Result` type, which is either `Ok(T)` if the parsing was successful, or `Err(E)` if there was an error. The `unwrap()` method is used to extract the value inside the `Result` type, assuming that the value is present and not an error.  In the example above, an explicit **type annotation** is used to indicate the type we want `parse()` to produce,

```rust
let n: i32 = data.parse().unwrap();
```

You can also use the `parse()` method like [this](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ccd3785fec96475e51c9e3fb6fce4efb)without an explicit **type annotation**,

```rust
fn main() {
    let data = "2024";
    let n = data.parse::<i32>().unwrap();
    println!("{}", n);
}
```