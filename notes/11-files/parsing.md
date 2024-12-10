# Parsing Strings - split() method

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

Now we may want to **parse** each String in the  [vector](notes/05-vectors/vectors.md).  For example, the following

```rust
fn main() {
    let courses =  "English 85";
    let courses: Vec<&str> = courses.split(" ").collect();
    print!("{:?}", courses);
}
```

code uses the `split()` method to separate a **String** according the a single whitespace character, producing the output,

```
["English", "85"]
```