# Functions

In Visual Studio Code, create a folder called **Functions** and then open it. Now open a new Terminal window. 

## Exercises

Create the following functions and implement / use them in the **main** function.

## 02-0: Slope

Create a new file called *slope.rs* with the following source code

```rust
fn main(){
    println!("Slope between (1.0,2.0) and (3.0,6.0): {}", slope(1.0, 2.0, 3.0, 6.0));
}

fn slope(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    // Return the correct slope value
    x1 + y1 + x2 + y2
}
```

Write a function called **slope(x1: f64, y1: f64, x2: f64, y2: f64) -> f64** that calculates and **returns** an **f64** value representing the slope of a line passing through the points (x1,y1) and (x2,y2).

## 02-1: Hypotenuse

Create a new file called *hypotenuse.rs* with the following source code

```rust
fn main(){
    println!("Hypotenuse - base: 3; height: 4: {}", hypotenuse(3.0, 4.0));
}

fn hypotenuse(a: f64, b: f64) -> f64 {
    // Return the correct hypotenuse value
    a + b
}
```

Write a function called **hypotenuse(a: f64, b: f64) -> f64** that calculates and **returns** an **f64** value representing the hypotenuse of a right triangle with sides a and b.

## 02-2: Distance

Create a new file called *distance.rs* with the following source code

```rust
fn main(){
    println!("Distance between (1,2) and (7,10): {}", distance(1.0, 2.0, 7.0, 10.0));
}

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    // Return the correct distance value
    x1 + y1 + x2 + y2
}
```

Write a function called **distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64** that calculates and **returns** an **f64** representing the distance between the points (x1,y1) and (x2,y2).

## 02-3: Prime

Create a new file called *prime.rs* with the following source code

```rust
fn main(){
    println!("Is 15 prime? {}", is_prime(15));
}

fn is_prime(num: u32) -> bool {
    // Return the correct boolean value: true or false
    // depending upon whether the variable num is a prime number
    num == 1
}
```

Write a function called **is_prime(num: u32) -> bool** that determines whether an integer **num** is prime and **returns** a **bool** value ... either **true** or **false** . 

## 02-4: Greatest Common Divisor

Create a new file called *gcd.rs* with the following source code

```rust
fn main(){
    println!("GCD of 48 and 36: {}", gcd(48, 36));
}

fn gcd(m: u32, n: u32) -> u32 {
    // Return the correct greatest common divisor
    // between m and n
    m+n
}
```
Write a function called **gcd(m: u32, n: u32) -> u32** that **returns** a **u32** representing the greatest common divisor of the numbers m and n, assuming m > n. The best-known algorithm for finding a greatest common divisor is Euclid’s Algorithm.

Euclid’s Algorithm states that the greatest common divisor of two integers m and n is 

* n if n divides into m completely.
* However, if n does not divide into m completely, then the answer is the **greatest common divisor** of n and the **remainder** of m divided by n.
