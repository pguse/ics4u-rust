# Using Option and Result types in Rust

In Visual Studio Code, create a folder called **Option-Result** and then open it. Now open a new Terminal window.

## Exercises

Complete the following exercises.

## Option Type


## 15-0:  Validating User Input

Create a program that takes an optional age input from a user. If the user provides an age, print `"You are X years old"`. If no age is provided, print `"Age not provided"`.

**Challenge:** Try to make the input dynamic using `stdin`!

## 15-1:  Calculating Discount

Write a program to calculate a discount based on an optional coupon code. If a user provides a coupon (`Some(coupon)`), apply a 10% discount. If no coupon (`None`), apply no discount.

## 15-2:  Summing Numbers

Create a function that takes a list of `Option<i32>` values and returns the sum of all `Some` values while ignoring `None`. - [Answer](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=261ba75f9fdb0eaf8797e7caef1773a1)

**Challenge:** Modify the program to handle negative numbers differently (e.g., skip them or replace with zeros).


## Result Type


## 15-3:  Division with Error Handling

Create a function that divides two numbers and returns a `Result`. If the divisor is zero, return an error. Otherwise, return the result of the division.

## 15-4:  Parsing a Number

Write a program that parses a string into an integer. If the string cannot be parsed, return an error. Otherwise, return the integer.

## 15-5: Solve a Quadratic

```rust
fn solve(a: i32, b: i32, c: i32) -> Result< (f64,f64), String > {
    // Add code here that implements
    // the quadratic formula
}

fn main() {
    match solve(1,-1,-12) {
        // Add code here that outputs a value
        // or 'No Real Solution'
    }
}
```

Based on the code [above](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=4f3c18f0dc2cef2c3b8dac39ead2ad83), finish the function called `solve(a: i32, b: i32, c: i32) -> Result<(f64,f64), String>` that takes three parameters representing the coefficients of a quadratic equation:  and returns either the value of the solution or the string 'No Real Solution'.  Notice that this example uses the **Result** enum instead of the **Option** enum.  Provide **two examples** in the `main()` function that demonstrate a real solution and a no real solution case.