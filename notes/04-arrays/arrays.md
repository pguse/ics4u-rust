# Arrays

An array must contain a single type of value, as stated earlier.  To iterate through an array, the best way is to avoid having to use index values, since a common error is to miss values at the ends of the array.  Instead, you can use the for loop like this

```rust
fn main() {
	let marks = [80, 72, 85, 83, 90, 95, 75, 88];

	print!("Marks: ");
	for m in marks {
		print!("{} ", m);
	}
	
	println!();
}
```
producing the output,

```
Marks: 80 72 85 83 90 95 75 88
```
The **for in** construct is used here to iterate through the array.  This is the preferred way to create a **for-loop** in Rust instead of using a range of index values.  This method is still shown below, since we saw this loop variation in Python.

[Reference:  Rust by Example - for loops](https://doc.rust-lang.org/rust-by-example/flow_control/for.html)

Of course, if you need to use the index you can iterate through the array as we first did in Python.  For example,

```rust
fn main() {
	let marks = [75, 90, 85, 95, 80, 92];

	print!("Marks: ");
	for i in 0..marks.len() {
		print!("{} ", marks[i]);
	}
	println!();
}
```
In this case, the **len()** function tells you the number of elements in the array.

To access both the index and the elements without looping through the indices, you can do this instead (this is preferable to the previous method). In this case an iterator is create using the **iter()** method and then the enumerate() method returns both the index and the corresponding value

```rust
fn main() {
	let marks = [75, 90, 85, 95, 80, 92];

	print!("Marks: ");
	for (_i, m) in marks.iter().enumerate() {
		print!("{} ", m);
	}
	println!();
}
```
In this case the variable `_i` stores the index and `m` stores the corresponding element.  The **underscore character** in the variable `_i` is necessary, because it indicates to the Rust compiler that we are not actually using the variable **(remove the underscore from `_i` and see what happens)**.  In this context, that probably doesn't make sense.  We would only use this method if we needed to use the index.  For example,

```rust
fn main() {
	let marks = [75, 90, 85, 95, 80, 92];

	print!("Marks: ");
	for (i, m) in marks.iter().enumerate() {
		print!("{}: {} ", i, m);
	}
	println!();
}
```
producing the output,

```
Marks:
0: 75
1: 90
2: 85
3: 95
4: 80
5: 92
```