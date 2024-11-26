# Functions

Our focus in this course is to create modular code.  One way we can accomplish this is to organize our code into functions.  Here is an example ***(see the given file - functions.rs)*** of using a function in our student information program:

```rust
fn main() {
    let marks = [80, 70, 90, 85, 75];
    
    println!("The average mark is: {}", average(marks));
    println!("All marks: {:?}", marks);
}

fn average(arr: [i32; 5]) -> i32 {
    let mut total = 0;

    for value in arr {
        total += value;
    }
    total / (arr.len() as i32)
}
```
Notice the creation of the function:  **average**.  It has a few characteristics that need to be pointed out.  Notice that in this example there is one input **parameter** required ... an **array of _i32_ integer values of size 5** called **arr** _(declared within the parentheses)_.  Notice the **return type** ... in this case **i32** _(declared after the parentheses)_.  The value of the calculation is returned from the function without using the keyword **return**.  Rust functions **automatically** return the last value in the function. If you look in the **main** function, the **average** function acts like a variable except that an array called **marks** is passed into it and **copied** to variable **arr**.  Its return value is output using the **println!** macro.

Functions can exist that do not return values.  An example is the **main** function we have been using in all our programs.  It has neither any input **parameters** provided or values returned.
