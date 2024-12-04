# Vectors

A vector is a collection type that allows you to store more than one value in a single data structure and puts all the values next to each other in memory. Vectors can only store values of the same type. They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.  Unlike an [array](/notes/04-arrays/arrays.md), **a vector can change its length**_.

## Creating a New Vector

To create a new empty **vector**, we call the **Vec::new()** function,

```rust
let v: Vec<i32> = Vec::new();
```

Notice that we are using a **type annotation** here (```Vec<i32>```) because we have not started with any values in the vector.  Often, you may want to start with initial values in the **vector** and in this case it is more convenient to the the built-in ```vec!``` **macro**.  For example,

```rust
let mut v = vec![5, 10, 15];
```

## Adding Elements to a Vector

Elements are appended to a vector by using the **push()** method.  For example,

```rust
let mut v: Vec<i32> = Vec::new();
v.push(5);
v.push(10);
v.push(15);
```

Notice the the vector must be made **mutable** using the keyword **mut**, in order to change its number of elements.  The **vector** can now be displayed as we display an array using,

```rust
println!("{:?}", v);
```

producing the output,

```
[5, 10, 15]
```

Similar to an [array](/notes/04-arrays/README.md), we access elements of a vector using square brackets and an **index**,

```rust
println!("{}", v[1]);
```

producing,

```
10
```

We can also modify an element of a **vector** using the same index notation.  For example,

```rust
v[1] = 100;
println!("{:?}", v);
```

produces the output,

```
[5, 100, 15] 
```

We can also loop through a vector as we do an array,

```rust
for i in 0..v.len() {
    print!("{} ", v[i])
}
```

producing,

```
5 10 15
```

## Working with Vectors and Arrays

Instead of adding elements one by one to a vector using the ```push``` method, you may want to add elements already stored in an array.  Here is an example of how to do this using the ```extend_from_slice``` method,

```rust
let mut v = vec![5, 10, 15];
let a = [20, 25, 30];
v.extend_from_slice(&a);
println!("{:?}", v);
// a is unchanged
println!("{:?}", a);
```

producing the output,

```
[5, 10, 15, 20, 25, 30]
[20, 25, 30]
```

Notice how ```a``` remains unchanged and how we passed in a slice _(using the reference symbol: &)_ to the **extend_from_slice** method.

## Combining Vectors

Sometimes we want to combine vectors together.  There are a number of ways to do this.

### Method #1: concat()

The following [code](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=3ea40e77f575a8d9fe437a5fc01ed24c)

```rust
fn main() {
	let a = vec![1, 2, 3, 4];
	let b = vec![5, 4, 3, 2, 1];
	let c = [a, b].concat();
	println!("{:?}", c);
}
```

uses the `concat()` method to concatenate _(stick together)_ two vectors to form a third vector

```
[1, 2, 3, 4, 5, 4, 3, 2, 1]
```

You can concatenate as many vectors as you want with the `concat()` method.
### Method #2: append

The following code,

```rust
let mut v = vec![5, 10, 15];
let mut w = vec![20, 25, 30];
v.append(&mut w);
// notice: w is emptied, but still exists
println!("{:?}", v);
println!("{:?}", w);
```

produces the output,

```
[5, 10, 15, 20, 25, 30]
[]
```

Vector ```w``` is appended to vector ```v```, and in the process ```w``` is emptied.  Notice how both vectors are **mutable** and a **mutable reference** ```&mut``` has been passed to the ```append``` method.

### Method #3: extend (moving)

The following code,

```rust
let mut v = vec![5, 10, 15];
let w = vec![20, 25, 30];
v.extend(w);
println!("{:?}", v);
// w is moved and cannot be used anymore.  it does not exist.
```

produces the output,

```
[5, 10, 15, 20, 25, 30]
```

Vector ```w``` is appended to vector ```v```, and in the process ```w``` is **moved into** the ```extend``` method. The vector ```w``` no longer exists after that point.  Notice how only vector ```v``` needs to be **mutable** and the vector ```w``` has been passed _(or moved)_ to the ```extend``` method.

### Method #4: extend (borrowing)

The following code,

```rust
let mut v = vec![5, 10, 15];
let w = vec![20, 25, 30];
v.extend(&w);
println!("{:?}", v);
// w is unchanged
println!("{:?}", w);
```

produces the output,

```
[5, 10, 15, 20, 25, 30]
[20, 25, 30]
```

Since a **reference** to ```w``` is passed into the ```extend``` method, ```w``` is said to have been **borrowed** by ```extend```, and so still exists afterwards.

## Merging Arrays into a Vector

Sometimes you may want to combine two arrays/vectors into a vector in a more complicated way than simply using the ```append``` or ```extend```  The following code,

```rust
let a = [1,3,5,7];
let b = [2,4,6,8];

let mut c: Vec<i32> = Vec::new();

for i in 0..a.len() {
    c.push(a[i]);
    c.push(b[i]);
}

println!("{:?}",c);
```

produces the output,

```
[1, 2, 3, 4, 5, 6, 7, 8]
```

by combining **every other element** of two arrays together to form a new vector.  This code only works if the two arrays have the same size.  For two different sized arrays, the following code,

```rust
fn main() {
    let m = [1,3,5,7,9,11,13];
    let n = [2,4,6,8];
    
    let q = merge(&m,&n);
    
    println!("Merged: {:?}",q);
}

fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    // initializing two values at a time
    let (mut i, mut j) = (0,0);
    // creating a new empty vector
    let mut c: Vec<i32> = Vec::new();
    loop {
        if i < a.len() {
            // add an element using the push() method
            c.push(a[i]);
            i += 1;
        }
        if j < b.len() {
            c.push(b[j]);
            j += 1;
        }
        if i == a.len() && j == b.len() {break}
    }
    c
}
```

produces the output,

```
[1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 13]
```

Notice that **references** to the arrays ```m``` and ```n``` are passed to the **merge** function, instead of passing copies of the arrays, and a new **vector** is returned.  Note:  Both arrays ```m``` and ```n``` will exist in the code after the **merge** function whether we pass a copy or a reference into the function.  This is **not the case** for **vectors**.

## Merging Vectors into a Vector

As we did above with two arrays, two vectors _(of different sizes) can also be merged together using similar code,

```rust
fn main() {
    let m = vec![1,3,5,7,9,11,13];
    let n = vec![2,4,6,8];
    
    let q = merge(&m,&n);
    
    println!("Merged: {:?}",q);
}

fn merge(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    // initializing two values at a time
    let (mut i, mut j) = (0,0);
    // creating a new empty vector
    let mut c: Vec<i32> = Vec::new();
    loop {
        if i < a.len() {
            // add an element using the push() method
            c.push(a[i]);
            i += 1;
        }
        if j < b.len() {
            c.push(b[j]);
            j += 1;
        }
        if i == a.len() && j == b.len() {break}
    }
    c
}
```

It produces the same output,

```
[1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 13]
```

Because **references** to the vectors were passed into the **merge()** function, both vectors ```m``` and ```n``` are available after that point in the code.  They are said to have been **borrowed** by the **merge()** function.  If we had passed in the actual vectors, the **rust** compiler considers them to have been **moved** into the **merge()** function, and both vectors would not exist after that point in the code.