# Using Iterators in Rust

## Exercises using the `map()` method

In Visual Studio Code, create a new project for the following exercises using `cargo new iterators_map`

## 12-0:  Square Numbers

   - Given a vector of integers, use `map()` to create a new vector where each element is squared.
   - Example input: `vec![1, 2, 3, 4, 5]`
   - Expected output: `vec![1, 4, 9, 16, 25]`

## 12-1: Factorials

   - Given a vector of integers, use `map()` to create a new vector where each element is the factorial of the corresponding integer.
   - Example input: `vec![1, 2, 3, 4, 5]`
   - Expected output: `vec![1, 2, 6, 24, 120]`

## 12-2:  Uppercase Strings

   - Given a vector of strings, use `map()` to create a new vector where each string is converted to uppercase.
   - Example input: `vec!["hello", "world"]`
   - Expected output: `vec!["HELLO", "WORLD"]`

## 12-3: Extracting First Characters

   - Given a vector of strings, use `map()` to create a new vector where each element is the first character of the original string.
   - Example input: `vec!["apple", "banana", "cherry"]`
   - Expected output: `vec!['a', 'b', 'c']`

## 12-4: Convert to Floating Point

   - Given a vector of integers, use `map()` to create a new vector where each element is converted to a floating-point number.
   - Example input: `vec![1, 2, 3, 4, 5]`
   - Expected output: `vec![1.0, 2.0, 3.0, 4.0, 5.0]`

## 12-5: Convert to i32

   - Given a vector of strings, use `map()` to create a new vector where each element is converted to an `i32` value.
   - Example input: `vec!["1", "2", "3", "4", "5"]`
   - Expected output: `vec![1, 2, 3, 4, 5]`

## Exercises using the filter()` method

In Visual Studio Code, create a new project for the following exercises using `cargo new iterators_filter

## 12-6: Filtering Odd Numbers

   - Given a vector of integers, use `filter()` to create a new vector that contains only the odd numbers.
   - Example input: `vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]`
   - Expected output: `vec![2, 4, 6, 8, 10]`

## 12-7: Filtering Positive Numbers

   - Given a vector of integers, use `filter()` to create a new vector that contains only the positive numbers.
   - Example input: `vec![-10, -5, 0, 5, 10, 15]`
   - Expected output: `vec![5, 10, 15]`

## 12-8: Filtering Strings with Length Greater Than 3

   - Given a vector of strings, use `filter()` to create a new vector that contains only the strings with a length greater than 3.
   - Example input: `vec!["cat", "dog", "elephant", "fox"]`
   - Expected output: `vec!["elephant"]`

## 12-9: Filtering Prime Numbers

   - Given a vector of integers, use `filter()` to create a new vector that contains only the prime numbers.
   - Example input: `vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11]`
   - Expected output: `vec![2, 3, 5, 7, 11]`