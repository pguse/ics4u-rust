# Arrays

In Visual Studio Code, create a folder called **Arrays** and then open it. Now open a new Terminal window. 

## Exercises

## 04-0: Student Average

1.  Create a file in **Arrays** called **average.rs**.  Modify the starter code below so that it averages 8 marks given in an integer array.  You will calculate the average, by using a **for-loop**.  ***Note:  You will need to use the idea of using an accumulator that we used in grade 11 with Python.***
2.  Output both the marks ***(all on one line)*** and the average, as shown in the example below.

```rust
fn main() {

	let marks = [75, 82, 90, 95, 87, 80, 70, 92];
	let mut sum = 0;

	// Simple array output - You can comment this out
	println!("{:?}", marks);

	for _i in 0..marks.len() {
		// Update the accumulator
		sum = marks[0];
	}

	print!("Marks:  ");
	// Insert a Loop
	print!("{} ", marks[0]);

	// Correct this calculation
	let average = sum as f64;
	println!("\nAverage:  {:0.1}", average);
}
```


**Example Output:**
```
Marks:  72 85 67 78 90 81 87 75
Average:  79.4
```

## 04-1: Reverse Output

Create a file in **Arrays** called **reverse_output.rs**.  Modify the starter code below so that it displays the given array in reverse order.

```rust
fn main() {

	let values = [2, 5, 7];

	for _i in (0..values.len()).rev() {
        print!("{} ", values[0]);
    }
}
```

For a given array: [2 5 7]

**Example Output**
```
7 5 2
```

## 04-2: Copying Elements Array-to-Array in Reverse Order

Create a file in **Arrays** called **array_copy_reverse.rs**.  Modify the source code below to copy the elements in one array into another array, but in reverse order.  

```rust
fn main() {

	let values = [2, 8, 5, 10];
	let mut reverse_values = [0; 4];

    
	for i in 0..values.len() {
		reverse_values[i] = values[i];
	}

	// Insert Loop
	print!("{} ", reverse_values[0]);
}
```

For the given array: [2, 8, 5, 10]

**Example Output**
```
10 5 8 2
```

## 04-03: Maximum and Minimum Elements in an Array

Create a file in **Arrays** called **max_min.rs*.  Modify the starter code below to find the maximum and minimum element in a given array of positive integers.  

```rust
fn main() {

	let values = [40, 25, 15, 70, 60];
	let mut max = values[0];
	let mut min = values[0];

	for i in 0..values.len() {
		// Insert an if-statement
		max = values[i];
		// Insert an if-statement
		min = values[i];
	}

	println!("Maximum:  {}", max);
	println!("Minimum:  {}", min);
}
```

For a given array: [40, 25, 15, 70, 60]

**Example Output**
```
Maximum: 70
Minimum: 15
```
