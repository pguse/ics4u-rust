# Functions

Our focus in this course is to create modular code.  One way we can accomplish this is to organize our code into functions.  Here is an example ***(see the given file - functions.rs)*** of using a function to calculate the area of a rectangle:

```rust
fn main() {
    let length = 4;
    let width = 3;
    println!("Area: {}", area_rectangle(length,width));
}

fn area_rectangle(base: i32, height: i32) -> i32 {
    base * height
}
```
Notice the creation of the function:  **area_rectangle()**.  It has a few characteristics that need to be pointed out.  Notice that in this example there are two input **parameter**s required ... two  _i32_ integer values.  Notice the **return type** ... in this case **i32** _(declared after the parentheses)_.  The value of the calculation is returned from the function without using the keyword **return**.  Rust functions **automatically** return **the last value** in the function. If you look in the **main()** function, the **area_rectangle()** function acts like a variable except that two **i32** integers **length** and **width** are passed into it and **copied** to the variables **base** and **height**.  Its return value is output using the **println!()** macro.

Functions can exist that do not return values.  An example is the **main()** function we have been using in all our programs.  It has neither any input **parameters** provided or values returned.  Of course, we can also create our own functions without a return value.  For example ***(see the given file - functions1.rs)***,

```rust
fn main() {
	let faculty_info = "Paul Guse, BSc, MSc, BEd (he/him)".to_string();
	let job_title = "Faculty - Mathematics and Computer Science".to_string();
	signature(faculty_info, job_title);
}

  

fn signature(info: String, title: String) {
	println!("{}\n{}", info, title);
	println!("Albert College - Belleville, Ontario");
}
```
the function **signature()** takes two **parameters** of type **String** as input, and does not return any value.


