# 2D Arrays

In Visual Studio Code, create a folder called **2d-arrays** and then open it. Now open a new Terminal window. 

## Exercises

## 08-0:  Random Matrix

Using **cargo**, create a new project called **random_matrix**. In **main.rs**, create a function called **random_matrix()** that creates a 2-dimensional array, with **3** rows and **3** columns, made up of 0's and 1's chosen randomly.  The function should return the array that gets created.

```rust
fn random_matrix() ->[[i32;3];3] {
    [[0,0,0],[0,0,0],[0,0,0]]
}
```

## 08-1:  Find

Create a function called **find()** that searches for an integer **x** in a 3x3 integer array **m**.  It should return the position **(row, col)** if it finds it, otherwise it returns **(-1,-1)**.

```rust
fn find(m: &[[i32; 3]; 3], x: i32) -> (i32,i32) {
    let loc = (-1, -1);
    // Search for x using a nested loop
    loc
}
```

## 08-2:  Maximum

Create a function called **max()** that returns the maximum value in a 3x3  integer array **m**.

```rust
fn max(m: &[[i32; 3]; 3]) -> i32 {
    0
}
```

## 08-3:  Addition

Create a function called **add** that adds two 3x3 integer **2D arrays** and returns a new 2-dimensional array. **Addition of arrays** produces a new array of the same size by adding **elements** from the original arrays with the **same row and column**, and placing the **sum** in the **same position** in the new array.

```rust
fn add(m: &[[i32; 3]; 3], n: &[[i32; 3]; 3]) -> [[i32; 3]; 3] {
}
```

## 08-4:  Scalar

Create a function called **scalar()** that multiplies every value of a 3x3 2D array **m** by the value **k**.

```rust
fn scalar(m: &mut [[i32; 3]; 3], k: i32) {
}
```
