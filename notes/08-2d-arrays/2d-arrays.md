# 2-Dimensional Arrays

A **1 dimensional array** is a **linear** representation of elements.  It represents a type of array that can be accessed by subsequent memory locations. For example:

```rust
let v = [4, 5, -3, 0, 7];
```
where each element of the array can be accessed by an ordered **index** starting from 0.  For example:

```rust
v[3]
```

A **2 dimensional array** is a representation of items in the form of **rows** and **columns**.  This is called a **tabular** representation of data.  Elements in a 2D array are accessed by using a **row** and **column** index.  Another way to think of a 2D array, is an array of arrays.  For example,

```rust
let m = [[1, 2, 3], [3, 4, 5], [6, 7, 8]];

println!("{:?}", m);
```

The output for the above code is

```
[[1 2 3] [3 4 5] [6 7 8]]
```

You can also create a **2D array** by writing the **rows** on **separate lines** as follows:

```rust
let m = [   [1, 2, 3],
            [3, 4, 5],
            [6, 7, 8]];

println!(m);
```

The output is unchanged.

## Accessing an Element
An element in a **2D array** is accessed by using the **row** and **column** values as follows:

```rust
println!("{}", m[2][1]);
```

produces the output

```
7
```

## Using a For Loop
A for-loop can be used to scan through the rows and columns of a 2 dimensional array or generate the values of the elements.

### Adding the elements

The following code adds up the elements of a 3x3 array.

```rust
fn total(a: [[i32;3];3]) -> i32 {
	let mut s = 0;
	for row in 0..3 {
		for col in 0..3 {
			s += a[row][col];
		}
	}
	s
}
```

The code above makes a copy of the array.  If you simply want to pass an **immutable reference** to the function, it would look like this,

```rust
fn total(a: &[[i32;3]]) -> i32 {
	let mut s = 0;
	for row in 0..3 {
		for col in 0..3 {
			s += a[row][col];
		}
	}
    s
}
```

since we are dealing with an array of arrays *(with 3 i32 values)*.

### Generating elements

You can generate a **random** array of elements by using a for-loop and the **rand** crate.  The following code creates a 3x3 array of random values from 1->9 inclusive, in two different ways; one way uses a function.

```rust
use rand::Rng;

fn main() {
	let mut n = [[0;3];3];

	// Create a random 2D array with values 1->9
	for row in 0..3 {
		for col in 0..3 {
			n[row][col] = rand::thread_rng().gen_range(1..=9);
		}
	}

	println!("{:?}", n);
	println!("{:?}", init_array());
}

// Return a random 2D array with values 1->9
fn init_array() -> [[i32;3]; 3] {
	let mut a = [[0;3];3];
	for row in 0..3 {
		for col in 0..3 {
			a[row][col] = rand::thread_rng().gen_range(1..=9);
		}
	}
	a
}
```

The **gen_range(1..=n)** function returns, as an **i32**, a non-negative pseudo-random number in the closed interval [0,n].

## Slices

A slice of any array can be accessed using the following notation

```rust
let v = [4, 5, -3, 0, 7];
let w = &v[2..4];
println!("{:?}", w);
```

The output of this code is

```
[-3 0]
```

The identifier **w** is a **slice** of array **v** from indices 2 **up to**, but not including, 4.  If you would like to create a slice up to/from one of the ends of the array, just remove one of the index values.  For example, 

```rust
let v = [4, 5, -3, 0, 7];
let x = &v[2..];
println!("{:?}",x);
```

produces the output

```
[-3 0 7]
```

and

```rust
let v = [4, 5, -3, 0, 7];
let y = &v[..4];
println!("{:?}",y);
```

produces the output

```
[4 5 -3 0]
```

## Slices of 2-Dimensional Arrays

This notation can also be used to access any number of rows or columns of a 2-dimensional array.  For example, we can access the first two **rows** of a 2D array as follows,

```rust
let magic = [   [2, 7, 6],
                [9, 5, 1],
                [4, 3, 8]];

println!("{:?}",&magic[0..2]);
```

This produces the output

```
[[2 7 6], [9, 5, 1]]
```

The expression ```magic[0..2]``` creates a **slice** of the magic array consisting of **row 0** and **row 1**.  **Note:**  Using ```[..]``` creates a slice of an array from its beginning to its end _(both index values are missing)_.

**Note:**  We cannot access the columns of a 2D array using slices because the data is stored as an **array of arrays**, where each array represents a row.

If you only want to access a single row, you can either use

```
    println!("{:?}",magic[0]);
```
or
```
    println!("{:?}",&magic[0]);
```

Both produce the output

```
[2 7 6]
```

## Dimensions of a 2-Dimensional Array

Assuming we are dealing with a rectangular 2D array, we can calculate the dimensions as follows,

```rust
let matrix = [  [2, 7, 6, 5],
                [9, 5, 1, 0],
                [4, 3, 8, 2]];

println!("# Rows: {}", matrix.len());
println!("# Columns: {}", matrix[0].len());
```

This produces the output

```
# Rows: 3
# Columns: 4
```
