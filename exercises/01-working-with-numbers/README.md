# Working with Numbers

In Visual Studio Code, create a folder called **Numbers** and then open it. Now open a new Terminal window. 

## Exercises

## 01-0: Student Average

Create a new file called *average.rs* with the following source code

```rust
fn main() {
    let m1 = 80.0;
    let m2 = 90.0;
    let m3 = 96.0;

    let average = (m1 + m2 + m3) / 3;

    println!("Average: {}", average);
}
```

3.  Type ***rustc average.rs*** into the Terminal.  Something isn't right.  Everything seems fine, but **Rust** will not perform division of different types:  a **float** (f64) and an **integer** (i32).  Change the 3 to a 3.0 and try again to compile the program.  You should now be able to run the program by typing: 1. (On a Mac) **average**, 2. (On Windows) **./average**.
4.  Format the output average with **two decimal places** by changing the {} in the **println!** statement to {:.2}.  Note:  In Rust, println! is an example of a **macro**, not a function.  We will discuss the difference later.

## 01-1: Volume of a Cylinder
In Visual Studio Code, create a new file in **Numbers** called *cylinder_volume.rs*.  Use the source code provided below as starter code in your file.  Modify the code so that it calculates the volume of a cylinder <img src="https://render.githubusercontent.com/render/math?math=V = \pi^2 rh"> instead of the area of a circle.  **Note:** Pay attention to the use of **consts::PI** to represent the value of pi and **i32::pow()** to perform the exponentiation.  See how the line ```use std::f64::consts;``` imports **std::f64::consts** into your program.

1. Before modifying the given code, type ***rustc cylindar_volume.rs*** into the Terminal.  Something isn't right. **Rust** will not perform multiplication of a **f64** type and an **i32** type.  There are two ways to fix this. Replace ```i32::pow(radius, 2)``` with ```i32::pow(radius, 2) as f64```.  This is an example of casting an **i32** type to an **f64** type.  Otherwise, your could just convert all values to **floating-point** types:  Change ```let radius = 5``` to ```let radius = 5.0```, and change ```i32::pow(radius,2)``` to ```f64::powf(radius,2.0)```.
2. Now modify the code to calculate the volume of a cylinder.

```rust
use std::f64::consts;

fn main() {
    let radius = 5;

    let area = consts::PI * i32::pow(radius, 2);

    println!("Area of the Circle: {}", area);
}
```

## 01-2: Hypotenuse of a Right-Angled Triangle
In Visual Studio Code, create a new file in **Numbers** called *hypotenuse.rs*.  Modify the starter code below so that it calculates the correct hypotenuse of a triangle with sides **a** and **b**.

1. Before modifying the given code, type ***rustc hypotenuse.rs*** into the Terminal.  Something isn't right. **Rust** will run the **sqrt()** function on the variable **sum**.  It is not the correct **type**.  To correct this problem, change the expression ```sum.sqrt()``` to ```(sum as f64).sqrt()```.  This is another example of **casting**.
2. Now modify the code to calculate the correct hypotenuse.

```rust
fn main() {
    let a = 3;
    let b = 4;

    let sum = a + b;
    let hypotenuse = sum.sqrt();

    println!("Hypotenuse: {}", hypotenuse);
}
```

## 01-3: Swap Digits *(using the modulus % operator)*
In Visual Studio Code, create a new file in **Numbers** called *swap.rs*.  Modify the starter code below so that it swaps the digits of the variable *number*. You should save the new value in a variable called *swap* and output its value use the macro **println!**.  *Assume that the number you are swapping only has two digits.*  **Note:** The */* operator returns the **quotient** of a division of two integers, while the **modulus** operator *%* returns the **remainder** of a division of two integers.

```rust
fn main() {
    let number = 75;
    let swap: i32;

    println!("Tens: {}", number/10);
    println!("Remainder: {}", number%10);
}
```