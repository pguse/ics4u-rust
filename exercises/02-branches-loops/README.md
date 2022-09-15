# Branches and Loops

In Visual Studio Code, create a folder called **BranchesLoops** and then open it. Now open a new Terminal window. 

## Exercises

## 02-0: At Least One Odd Number?
In Visual Studio Code, create a new file in **BranchesLoops** called *one_odd.rs*.  Use the source code provided below as starter code in your file.

```rust
fn main() {
	// Create variables with values
	let a = 10;
    let b = 15;

	// Is at least one value even?
	// Note != is the NOT EQUAL TO operator
	if a%2 == 0 || b%2 == 0 {
		println!("YES");
	} else {
		println!("NO");
	}
}
```

Modify the source code so that it determines if at **least one of 3** given numbers is **odd**.  Currently, the source code is checking whether at least one of 2 numbers is even.

**Output:**  print "YES" if at least one of 3 numbers is odd and print "NO" otherwise.


## 02-1: Ascending Integers?

In Visual Studio Code, create a new file in **BranchesLoops** called *ascendingIntegers.go*.  Use the source code provided below as starter code in your file.

```go
package main

import "fmt"

func main() {
	// Declare another variable with its value
	a, b, c := 10, 15, 20

	// Are these values descending?
	if a > b && b > c {
		fmt.Println("YES")
	} else {
		fmt.Println("NO")
	}
}
```

Modify the source code so that it determines whether 4 variables: *a*, *b*, *c*, and *d* contain **ascending integer values**.

 **Output:** print YES if they're given in ascending order, print NO otherwise.


## 02-2: Multiplication Table  
In Visual Studio Code, create a new file in **BranchesLoops** called *multTable.go*.  Use the source code provided below as starter code in your file.

```go
package main

import "fmt"

func main() {
	number := 5

	for i := 1; i <= 10; i++ {
		fmt.Printf("%d x %d = \n", number, i)
	}
}
```
Modify the source code so that the multiplication table for the variable *number* is printed out up to 10.  For example,

### Expected Output:
```
  5 x 1 = 5
  5 x 2 = 10
  5 x 3 = 15
  5 x 4 = 20
  …
  5 x 10 = 50
```
## 02-3: Divisors
In Visual Studio Code, create a new file in **BranchesLoops** called *divisors.go*.  Modify the starter code below so that it outputs **all the divisors** of a number given as input.

```go
package main

import "fmt"

func main() {
	number := 12

	for i := 1; i <= number; i++ {

		// Is the remainder of (number / i) equal to zero?

		fmt.Println(i)
	}
}
```

For *number := 12*

### Expected Output:
```
  1
  2
  3
  4
  6
  12
```

## 02-4: Perfect Numbers
In Visual Studio Code, create a new file in **BranchesLoops** called *perfectNumber.go*.  The **divisors** of a **perfect number** ***(except for the number itself)*** add up to the number. For example: 6 has divisors 1, 2, 3, 6 and 1 + 2 + 3 = 6. So, 6 is a perfect number. Modify the starter code so that it determines whether a number is **perfect**.

```go
package main

import "fmt"

func main() {
	number := 12
	sum := 0

	for i := 1; i < number; i++ {

		// Is the remainder of (number / i) equal to zero?

		sum = sum + i
	}

	if sum == number {
		fmt.Printf("The number %d is a perfect number.", number)
	}

}
```

For *number := 6*

### Expected Output:
```
  The number 6 is a perfect number.
```

For *number := 10*

### Expected Output:
```
  The number 10 is NOT a perfect number.
```