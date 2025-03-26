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

Create a function that takes a list of `Option<i32>` values and returns the sum of all `Some` values while ignoring `None`.

**Challenge:** Modify the program to handle negative numbers differently (e.g., skip them or replace with zeros).


## Result Type


## 15-3:  Division with Error Handling

Create a function that divides two numbers and returns a `Result`. If the divisor is zero, return an error. Otherwise, return the result of the division.

## 15-4:  Parsing a Number

Write a program that parses a string into an integer. If the string cannot be parsed, return an error. Otherwise, return the integer.