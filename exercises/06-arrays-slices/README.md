# Arrays - The Slice Type

## Note

Sometimes we want to access more than one element of an array.  In this case, we need to use more than one index.  This is done using a [**slice**](https://doc.rust-lang.org/book/ch04-03-slices.html#other-slices). For [example](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a9564d53609a6e688cc81eb1e6013c56),

```rust
fn main(){
    let a = [10, 20, 30, 40];
    // The following statements output
    // a slice of the array a
    println!("{:?}", &a[1..3]);
    println!("{:?}", &a[..2]);
    println!("{:?}", &a[1..]);
    println!("{:?}", &a[..]);
    println!("{:?}", &a);
}
```

produces the output

```
[20, 30]
[10, 20]
[20, 30, 40]
[10, 20, 30, 40]
[10, 20, 30, 40]
```

Notice how array slices are creating using two values: the **initial** and **final index**, but the values only go up to *(but do not include)* the final index.  If you leave off either value, the slice goes entirely to the the beginning or end of the array.  If you leave off both indices ```&a[..]```, you create **a slice of the entire array**.  You can also use ```&a``` to create a slice of the entire array.  But, why would you want to do that?

When creating **functions** that take an array as a **parameter**, the array type includes both the type of each **value** as well as the **number of elements** in the array.  For example, the following program

```rust
fn main(){
    let first_array = [85, 90, 92, 80, 99];

    println!("{:?}", minimum(first_array));
}

fn minimum(a: [i32;5]) -> i32 {
    let mut min = a[0];
    for i in 0..a.len() {
        if a[i] < min {
            min = a[i];
        }
    }
    min
}
```

uses a **minimum()** function that takes a 5-element array of i32 values as a parameter. However, it would be more useful if the **minimum()** function could find the minimum value of **any length array** of **i32** values.  We can create this version of the function using an array slice.  For example,

```rust
fn main(){
    let first_array = [85, 90, 92, 80, 99];
    let second_array = [300, 100, 200];

    println!("{:?}", minimum(&first_array));
    println!("{:?}", minimum(&second_array));
}

fn minimum(a: &[i32]) -> i32 {
    let mut min = a[0];
    for i in 0..a.len() {
        if a[i] < min {
            min = a[i];
        }
    }
    min
}
```

gives the output

```
80
100
```

The **minimum()** function can handle any length array because its parameter is an **array slice**.  Another way of describing the parameter is that it is a **reference** to the array.  Notice how the syntax is different for the function **parameter**,

```rust
fn minimum(a: &[i32]) -> i32
```

but the function body remains unchanged.

The second benefit to this type of function is that a **copy** of the entire array is **not created** when the function is called.  Only a **reference** to an array is copied into *(borrowed by)* the function.

A **copy** of the array is created when using the **array** as an argument as opposed to a **reference to an array**.

## Mutable References

Sometimes you need to create a function that **modifies the values** of an array.  You can achieve this, **without a return value**, by using a **mutable reference** to an array as the function parameter.  For example,

```rust
fn main(){
    let mut first_array = [80, 85, 90, 92, 99];
    println!("{:?}", first_array);
    double(&mut first_array);
    println!("{:?}", first_array);
}

fn double(a: &mut [i32]){
    for i in 0..a.len(){
        a[i] = 2*a[i];
    }
}
```

produces the output,

```
[80, 85, 90, 92, 99]
[160, 170, 180, 184, 198]
```

Notice how the original array is modified, **without needing to return a copy** of the entire array.  This may not seem like a big deal when dealing with a 5-element array.  Imagine the array has a million values.  Making a copy to return would be a waste of both time and memory.

## Exercises

In Visual Studio Code, create a folder called **Arrays-Slices** and then open it. Now open a new Terminal window. 

## 06-0: Student Average - Immutable Reference

Create a file in **Arrays-Slices** called **average.rs**.  Modify the starter code below so that it averages the marks stored in the **marks** array using the **average()** function. Note:  You must use an **immutable reference** as a parameter in your **average()** function.

```rust
fn main() {

	let marks = [75, 82, 90, 95, 87, 80, 70, 92];

	println!("{:?}", marks);
	println!("Average:  {:0.1}", average(marks));
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