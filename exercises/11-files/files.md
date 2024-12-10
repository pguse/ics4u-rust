# Files
## Exercises

Complete the following exercises.

## 11-0:  Advent of Code 2024 - Day 2 - Part 1

**Read** the situation described on the [Advent of Code - Day 2 - Red-Nosed-Reports](https://adventofcode.com/2024/day/2).

### Task

Analyze the unusual data from the engineers provided in the input file provided here: `input.md`. 

### Question

_How many reports are safe?_ 

A report only counts as **safe** if **both** of the following are true:

- The levels are either _all increasing_ or _all decreasing_.
- Any two adjacent levels differ by _at least one_ and _at most three_.

### Note

1. Create a **new project** called `advent_day2` using `cargo`: `cargo new advent_day2`
2. You will need to input the data from the text file, `input.md`. Use the code provided in the note on [files](/notes/11-files/files.md).
3. You will need to parse a **String** using the `split()` method.  Read the documentation on `split()` [here](https://doc.rust-lang.org/std/string/struct.String.html#method.split). The **String** obtained from the file should become a vector of type `Vec<String>`. The `split()` method produces something called an `iterator`.  In order to convert it to a [vector](/notes/05-vectors/vectors.md) you must use the [collect()](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) method. The `collect()` method is often used to turn one collection into another collection by first converting the collection to an **iterator** and then eventually running the `collect()` method on it. See these [examples](https://rust-guide.com/en/documentation/iterators/collect).
4. You may want to create separate functions for the **increasing** and **decreasing** cases.
