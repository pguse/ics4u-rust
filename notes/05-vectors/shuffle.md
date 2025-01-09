# Shuffling a Vector

Shuffling a vector in Rust is quite straightforward with the help of the `rand` crate. Here's a step-by-step guide:

1. **Add the `rand` crate to your `Cargo.toml` file:**
```toml
[dependencies]
rand = "0.8"
```

2. **Import the necessary modules in your Rust file:**
```rust
use rand::seq::SliceRandom;
use rand::thread_rng;
```

3. **Shuffle the vector:**
```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let mut rng = thread_rng();
    vec.shuffle(&mut rng);
    println!("{:?}", vec);
}
```

In this example, we first import the `SliceRandom` and `thread_rng` modules from the `rand` crate. We then create a mutable vector and a random number generator. The `shuffle` method is called on the vector, which will randomize the order of its elements.