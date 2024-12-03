# Functions and Arrays

Consider the Rust [code](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=67c6758a3cfc6e547a877cb07241c1b9)  below,

```rust
fn main() {
	let my_marks: [i32; 5] = [80, 70, 90, 85, 75];
	println!("The average mark is {}", average(my_marks));
	println!("All marks: {:?}", my_marks);
}

fn average(marks: [i32;5]) -> i32 {
	let mut total = 0;
	for m in marks {
		total += m;
	}
	total / (marks.len() as i32)
}
```

with the output,

```
The average mark is 80
All marks: [80, 70, 90, 85, 75]
```

Notice in the **average()** function that the [array](/notes/04-arrays/README.md) type includes **both the type of elements and the number of elements**.  The problem with this requirement is that the function can only use an array of 5 elements, which is not the most useful function, as we would rather be able to calculate the average value of an array of any number of elements.

Note:  In this version of the **average()** function a separate **copy** of the array is created inside the function. In the **main()** function the array is bound to the name `my_marks`, while in the **average()** function a separate copy of the array is bound to the name `marks`.