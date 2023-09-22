# Introduction to Rust

Rust is a compiled programming language, unlike Python which is an interpreted language.  Before you run a Rust program, a compiler translates your code into the 1s and 0s that machines use.  It compiles all your code into a single _executable_ for you to run.  During this process, the Rust compiler can catch any of your typos or syntax mistakes.

Here is a simple Rust program

```rust
fn main() {
	println!("Hello World!");
}
```

And here is how you would run the program on **Windows**,

![Running a Rust Program](https://github.com/pguse/ics4u-rust/blob/main/notes/00-intro/hello.png "Running a Rust Program")

The only difference on a **Mac** is the executable is just called **hello**, without the **.exe** file extension.

In the code above, the **fn** keyword declares a function called **main()**.  The **main()** identifier is special because when you run a Rust program, it runs the code found in the **main()** function.  Without **main()**, the Rust compiler would report an error.

## Using the Terminal

We will be using the terminal to run our code. Here are some commands that you will need to remember.

### ls

The **ls** command lists the files and folders in the current directory.  For example, in my **12 Computer Science** folder, here are the current files and folders stored there.

![Using ls](https://github.com/pguse/ics4u-rust/blob/main/notes/00-intro/ls.png "Using ls")

### pwd

The **pwd** command lists the current folder you are in.  Hence, it stands for 'present working directory'.  For example,

![Using pwd](https://github.com/pguse/ics4u-rust/blob/main/notes/00-intro/pwd.png "Using pwd")

### cd

The **cd** command changes folders, hence it stands for 'change directory'.  If you want to move into a folder that is contained in your current folder, you use the cd <folder> command.  For example,

![Using cd](https://github.com/pguse/ics4u-rust/blob/main/notes/00-intro/cd.png "Using cd")

If you want to move 'up' a folder into the folder that contains the current one, you use the **cd ..** command.  For example,

![Using cd..](https://github.com/pguse/ics4u-rust/blob/main/notes/00-intro/cd_dot_dot.png "Using cd..")