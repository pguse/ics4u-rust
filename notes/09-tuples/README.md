# Tuples

In Rust, a `tuple` is a **collection** of values of **different types** grouped together, in contrast to an [array](notes/04-arrays/README.md) or [[notes/05-vectors/README|vector]] that groups the values of the **same type**. Tuples are useful for returning multiple values from a function or grouping related values without creating a custom **struct**. Each element in a tuple **can be of a different type**, and they are accessed by their position within the tuple.

Here's an example of a `tuple` in Rust:

```rust
let my_tuple: (i32, f64, char) = (42, 3.14, 'a');
```

In this example, `my_tuple` contains an integer, a floating-point number, and a character. You can access the elements of the tuple using **pattern matching**:

```rust
let (x, y, z) = my_tuple;
println!("x: {}, y: {}, z: {}", x, y, z);
```

or directly by **index**:

```rust
println!("First element: {}", my_tuple.0);
println!("Second element: {}", my_tuple.1);
println!("Third element: {}", my_tuple.2);
```

Tuples are a handy way to **group different types of values** together in Rust.  They are especially useful when you want to return multiple values from a function.  For example:

```rust
fn main() {
	println!("Midpoint of (1,6) and (-8,2): {:?}", midpoint(1,6,-8,2));
}

fn midpoint(x1: i32, y1: i32, x2: i32, y2: i32) -> (f64, f64) {
	let x_mid = (x1 + x2) as f64 / 2.0;
	let y_mid = (y1 + y2) as f64 / 2.0;
	(x_mid, y_mid)
}
```
The output for this is
```
Midpoint of (1,6) and (-8,2): (-3.5, 4.0)
```

Notice the **return type** of the midpoint() function and the use of `Debug` printing as with other collections such as **arrays** and **vectors**.
