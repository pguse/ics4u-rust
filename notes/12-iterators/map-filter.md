# map()

In Rust, the `map()` method is used to transform the elements of an iterator, such as a [vector](notes/05-vectors/vectors.md), by applying a specified closure or function to each element. It's a common and powerful way to process collections in a **concise** and **functional style**. The method replaces the need to create a loop.

### How `map()` Works

Here's how the `map()` method works:

1. **Syntax**:
   ```rust
   iterator.map(|x| /* transformation */)
   ```

2. **Transformation**: The closure or function passed to `map()` takes each element of the iterator, applies a transformation, and returns the new value.

3. **Returns a New Iterator**: The `map()` method does not change the original iterator. Instead, it returns a new iterator containing the transformed elements.

Here is an [example](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=46ecf50c2d259299c20692c3fdd511b1) of using `map()` with a [vector](notes/05-vectors/vectors.md):

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Use map() to create a new iterator where each element is doubled
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

    println!("Original: {:?}", numbers);
    println!("Doubled: {:?}", doubled);
}
```

with the output,

```
Original: [1, 2, 3, 4, 5]
Doubled: [2, 4, 6, 8, 10]
```

In this example:
- `numbers` is a vector containing the elements `[1, 2, 3, 4, 5]`.
- the `iter()` method is used to change the `numbers` vector into an iterator so that `map()` can be applied to it.
- `map(|x| x * 2)` creates a new iterator where each element is doubled.
- `collect()` is used to gather the transformed elements into a new vector called `doubled`.

The `map()` method is part of Rust's iterator trait and is useful for a wide range of transformations.

# filter()

The `filter()` method in Rust is used to create an iterator that only yields elements that satisfy a specified condition. This method is part of the `Iterator` trait and is very useful for processing collections in a functional style.

### How `filter()` Works

1. **Syntax**:
   ```rust
   iterator.filter(|&x| /* condition */)
   ```

2. **Condition**: The closure or function passed to `filter()` takes each element of the iterator and returns a boolean value (`true` or `false`). Only elements for which the closure returns `true` are included in the resulting iterator.

3. **Returns a New Iterator**: The `filter()` method does not modify the original iterator. Instead, it returns a new iterator that contains only the elements that satisfy the condition.

### Example

Here's an [example](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=95c7dd341135cba6e435969cbab21aac) of using `filter()` with a vector to keep only the even numbers:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Use filter() to create a new iterator with only even numbers
    let even_numbers: Vec<_> = numbers.iter().filter(|&x| x % 2 == 0).collect();

    println!("Original: {:?}", numbers);
    println!("Even: {:?}", even_numbers);
}
```

with the output,

```
Original: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
Even: [2, 4, 6, 8, 10]
```

In this example:
- `numbers.iter()` creates an iterator over the vector `numbers`.
- `filter(|&x| x % 2 == 0)` applies a condition to each element, keeping only the even numbers.
- `collect()` gathers the filtered elements into a new vector called `even_numbers`.

### Key Points

- **Immutability**: The original collection remains unchanged.
- **Functional Style**: `filter()` allows for concise and expressive code.
- **Chaining**: You can chain `filter()` with other iterator methods like `map()`, `fold()`, etc., for more complex operations.

The `filter()` method is a powerful tool for working with iterators in Rust, enabling you to easily process and transform collections based on specific conditions. 

## What is `Vec<_>`?

In Rust, `Vec<_>` is a shorthand type annotation used to indicate that the type of elements in the vector should be inferred by the compiler. The underscore `_` acts as a placeholder for the type, signaling the compiler to determine the type based on the context in which the vector is used.

### When to Use `Vec<_>`

1. **Type Inference**: When the compiler can determine the type of elements from the surrounding code, you can use `Vec<_>` to let it infer the type.
2. **Readability**: It simplifies the code by removing redundancy, especially when the type is evident from the context.

### Example

In our [code](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=46ecf50c2d259299c20692c3fdd511b1) above, using the `map()` method, 

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Use map() to create a new iterator where each element is doubled
    let doubled: Vec<_> = numbers.iter().map(|&x| x * 2).collect();

    println!("Original: {:?}", numbers);
    println!("Doubled: {:?}", doubled);
}
```

- `Vec<_>` tells the compiler to infer the type of elements in the vector `doubled`.
- Since `numbers.iter()` returns an iterator of references to `i32`, the compiler infers the type of `doubled` as `Vec<i32>`.

Using `Vec<_>` can make your code more concise and readable by leveraging Rust's powerful type inference capabilities.