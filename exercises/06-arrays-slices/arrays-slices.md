# Arrays - The Slice Type

## Exercises

In Visual Studio Code, create a folder called **Arrays-Slices** and then open it. Now open a new Terminal window. 

## 06-0: Student Average - Immutable Reference

Create a file in **Arrays-Slices** called **average.rs**.  Modify the starter code below so that it averages the marks stored in the **marks** array using the **average()** function. Note:  You must use an **immutable reference** as a parameter in your **average()** function.

```rust
fn main() {

	let marks = [75, 82, 90, 95, 87, 80, 70, 92];

	println!("{:?}", marks);
	println!("Average:  {:0.1}", average(&marks));
}

fn average() -> f64 {
    0.0
}
```

## 06-1: Opposite - Mutable Reference

Create a file in **Arrays-Slices** called **opposite.rs**.  Modify the starter code below so that it changes all the elements *(components)* of the **velocity** array to their opposites *(+ to -; - to +)*, using the **opposite()** function. Note:  You must use a **Mutable reference** as a parameter in your **opposite()** function.

```rust
fn main() {

	let velocity = [12,-5, 13];

	println!("Velocity: {:?}", velocity);
    // use the opposite() to change the
    // velocity array to its opposite
	println!("Opposite Velocity:  {:?}", velocity);
}

fn opposite() {
}
```

For example, the output should be,

```
Velocity: [12,-5,13]
Opposite Velocity: [-12,5,13]
```
