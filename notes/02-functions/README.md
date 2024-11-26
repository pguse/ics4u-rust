# Functions

Our focus in this course is to create modular code.  One way we can accomplish this is to organize our code into functions.  Here is an example ***(see the given file - functions.rs)*** of using a function in our student information program:

```go
package main

import "fmt"

func main() {
	marks := [3]float64{75, 80, 90} // Declare Array with 3 elements
	fmt.Printf("Your average is: %0.2f", mean(marks))
}

func mean(a [3]float64) float64 {
	sum := 0.0
	for i := 0; i < len(a); i++ {
		sum = sum + a[i]
	}
	return sum / float64(len(a))
}
```
Notice the creation of the function:  **mean**.  It has a few characteristics that need to be pointed out.  Notice that in this example there is one input **parameter** required ... an **array of floating-point values of size 3** called **a** _(declared within the parentheses)_.  Notice the **return type** ... in this case **float64** _(declared after the parentheses)_.  The value of the calculation is returned from the function using the keyword **return**.  If you look in the **main** function, the **mean** function acts like a variable except that an array called **marks** is passed into it and **copied** to variable **a**.  Its return value is output using the **Printf** function.

Functions can exist that do not return values.  An example is the **main** function we have been using in all our programs.  It has neither any input **parameters** provided or values returned.
