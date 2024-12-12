## Iterators

In Rust, the `Iterator` trait is implemented by many types, allowing them to produce a sequence of values. Here are some common types and collections that implement the `Iterator` trait that we have already looked at in this course:

1. **Standard Collections**:
   - **Vectors**: can use the methods: `iter()`, `iter_mut()`, and `into_iter()`.
   - **Arrays**: Arrays implement `IntoIterator`, allowing them to be used with iterators.

2. **Ranges**:
   - **Range**: `0..10` produces numbers from 0 to 9.
   - **RangeInclusive**: `0..=10` produces numbers from 0 to 10.

3. **Strings**:
   - **Chars**: `str.chars()` produces an iterator over the characters of the string.
   - **Lines**: `str.lines()` produces an iterator over the lines of the string.

Iterators in Rust provide a powerful and flexible way to work with sequences of values. We have seen how to use a **Range** iterator in a **for-loop** as in the following [code](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=50e9b22c2b7f6d48e552941407797644):

```rust
fn main() {
    for num in 0..4 {
        println!("{} ", num);
    }
}
```

In this example,  `num` is assigned to each value in the **Range** iterator and output as follows

```
0 1 2 3 
```

We have also seen how to use an iterator with a [vector](notes/05-vectors/vectors.md).  For example in the following [code](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6d802ca2dc58843c184a1f2b78730f54),

```rust
fn main() {
    let numbers = vec![0, 1, 2, 3, 4, 5];
    for n in numbers.iter() {
        print!("{} ", n);
    }
}
```

the `iter()` method creates an iterator that iterates over **references** to the elements of the vector, and `for` loop consumes the iterator, without consuming the original collection `numbers`. The original vector `numbers` remains unmodified and can be used after the loop. The output is

```
0 1 2 3 4 5
```

The previous code **is equivalent to** to this [program](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=83239e8e72b3a681bad4d100c72928a6)

```rust
fn main() {
    let numbers = vec![0, 1, 2, 3, 4, 5];
    for n in &numbers {
        print!("{} ", n);
    }
}
```

The syntax `&numbers`also creates an iterator that **borrows** each element of the [vector](notes/05-vectors/vectors.md). Rust implicitly calls `iter()` on the reference to the vector, so the type of `n` is still `&i32`. The original vector `numbers` remains unmodified and can be used after the loop.

## collect()

The `collect()` method is used to create a collection from an iterator.  When using `collect()` you must use an explicit **type annotation** so that the compiler knows what type of collection to create.  For example, the following [code](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=fe5931c36e8acfe8c8115133ef3cde34) creates a [vector](notes/05-vectors/vectors.md) from a **Range** iterator.

```rust
fn main() {
    let numbers: Vec<i32> = (0..20).collect();
    for n in &numbers {
        print!("{} ", n);
    }
}
```

with the output

```
0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19
```

This would be useful when creating a  [vector](notes/05-vectors/vectors.md) of many numbers with the need of a **for-loop**.