# flat_map()

The `flat_map` function is an iterator method in **Rust** that applies a function to each item of an iterator and flattens the result. This function is often used when you have an iterator of iterators _(vectors, strings, etc.)_ and you want to process the inner elements as a single sequence.  Here are some examples:

## Example - Flattening a Vector of Vectors

```rust
fn main() {
    let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let : Vec<i32> = v.into_iter().flat_map(|x| x.into_iter()).collect();
    println!("{:?}", flattened); // Output: [1, 2, 3, 4, 5, 6, 7, 8, 9]
}
```
## Example - Flattening a Vector of Strings