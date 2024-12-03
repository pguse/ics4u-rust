
# Display and Debug Printing

Variables of simple types in Rust can be printed with `{}` inside **println!**. This is called `Display` printing. But some variables won’t be able to use `{}` to print, and you need `Debug` printing. You can think of `Debug` printing as printing for the programmer because it usually shows more information—and is usually less pretty.

## Example - Display

 [Printing](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=0dc3d4ee060fb5048421d3ef4a8e7194) a value that may be an integer, floating, char, or String looks like this,

```rust
fn main() {
    let name = "Albert College";
    let first_year = 1857;

    println!("{} was founded in {}.", name, first_year);
}
```
with output,
```
Albert College was founded in 1857.
```
## Example - Debug

 [Printing](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f355bf617986c68bb439a44eddbfdba1) a more complex value/type looks like this,

```rust
fn main() {
    let marks: Vec<u8> = vec![75, 65, 80, 92, 78, 87];
    let student_info = ["Guse", "Paul", "Graham", "Faculty"];

    println!("Student Info: {:?}", student_info);
    println!("Marks: {:?}", marks);
}
```
with output,

```
Student Info: ["Guse", "Paul", "Graham", "Faculty"]
Marks: [75, 65, 80, 92, 78, 87]
```
## Summary

There are three ways to print:

- `{}` - `Display` print. More types have `Debug` than `Display`, so if a type you want to print can’t print with `Display`, you can try `Debug`.
- `{:?}` - `Debug` print. If there is too much information on one line, you can try `{:#?}`.

### Pretty Print

We haven't talked about this yet.  It will come up when we talk about the [struct](notes/07-structs/structs.md) type.

- `{:#?}` - `Debug` print, but pretty. Pretty means that each part of a type is printed on its own line to make it easier to read.