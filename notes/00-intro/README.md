# Introduction to Rust

Rust is a compiled programming language, unlike Python which is an interpreted language.  Before you run a Rust program, a compiler translates your code into the 1s and 0s that machines use.  It compiles all your code into a single _executable_ for you to run.  During this process, the Rust compiler can catch any of your typos or syntax mistakes.

Here is a simple Rust program

```rust
fn main() {
	println!("Hello World!");
}
```

And here is how you would run the program,

![Running a Go Program](https://github.com/pguse/ics4u-rust/blob/main/notes/00-intro/hello.png "Running a Rust Program")

In the code above, the **fn** keyword declares a function called **main()**.  The **main()** identifier is special because when you run a Rust program, it runs the code found in the **main()** function.  Without **main()**, the Rust compiler would report an error.