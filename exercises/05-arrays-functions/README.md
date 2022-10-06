# Arrays and Functions

In Visual Studio Code, create a folder called **Arrays-Functions** and then open it. Now open a new Terminal window. 

## Exercises

## 05-0: Student Average

Create a file in **Arrays-Functions** called **average.rs**.  Modify the starter code below so that it averages the marks stored in the **marks** array using the **average()** function.

```rust
fn main() {

	let marks = [75, 82, 90, 95, 87, 80, 70, 92];

	println!("{:?}", marks);
	println!("Average:  {:0.1}", average(marks));
}

fn average(m: [i32;8]) -> f64 {
    0.0
}
```

Notice how the parameter **m** in the **average()** function has a **data type** that includes both the data type of the array elements and the array size.

## 05-1: Minimum

Create a file in **Arrays-Functions** called **minimum.rs**.  Modify the starter code below so that it determines the minimum mark stored in the **marks** array using the **minimum()** function.

```rust
fn main() {

	let marks = [75, 82, 90, 95, 87, 80, 70, 92];

	println!("{:?}", marks);
	println!("Minimum:  {}", minimum(marks));
}

fn minimum(m: [i32;8]) -> i32 {
    0
}
```

## 05-2: Mutable Arrays

Create a file in **Arrays** called **mutable.rs**.  First, run the starter code given below.  Notice how it doesn't modify the values in the array.  Your task is to modify the starter code below so that it doubles the values in the original array, using the function **double()**.  In this example, a copy of **my_array** is created when it is passed as an **argument** to **double()**.  Since array **a** is a copy, modifying it does not alter the original array **my_array**.

Change the **double()** function so that it returns an **[i32;4]** type array. We want the output to look like this,

**Output**
```
[2, 4, 6, 8]
[4, 8, 12, 16]
```

```rust
fn main(){
    let my_array = [2, 4, 6, 8];// modify this line
    println!("{:?}", my_array);
    double(my_array);           // modify this line
    println!("{:?}", my_array);
}

fn double(mut a: [i32;4]){  // modify this line
    for i in 0..a.len(){
        a[i] = 2*a[i];
    }
    // Add code here
}
```

# Tic-Tac-Toe Exercises

These exercises demonstrate how you might use a single-dimensional array in Rust to store information in a grid-based game like Tic-Tac-Toe . In this case a 9 element array is used to store the 9 possible positions of the tic-tac-toe game. The index values of the array  match the grid positions given in the table below.

|   Tic  |  Tac   |  Toe   |
|:---:|:---:|:---:|
|  0  |  1  |  2  |
|  3  |  4  |  5  |
|  6  |  7  |  8  |


## Board Setup

The **board** array is filled initially with the values ```X```, ```O```, or ```-```.  In this game, this value will represent an empty square and be displayed as ```-```.  See the starter code below.

```rust
fn main() {
	let board = ['X', 'O', 'X', 'O', 'X', '-', '-', 'X', 'O'];

	display(board)
}

fn display(b: [char; 9]) {
    for i in 0..b.len() {
        if i % 3 == 0{
            print!("\n{}  ", b[i]);
        } else {
            print!("{}  ", b[i]);
        }
    }
    println!();
}

fn is_win(b: [char;9]) -> bool {
	false
}

fn is_tie(b: [char;9]) -> bool {
	false
}
```

## Tasks

Create a file in **Arrays-Functions** called **tic_tac_toe.rs** and add the starter code given above.

## 05-3:  Is there a Win?

Complete the **is_win()** function. It should return **true** if there is a win on the board.  Otherwise, it should return **false**. Remember, there are 8 possible ways to win in the tic-tac-toe game:  3 of the same kind _('X' or 'O')_ in any row _(3)_, any column _(3)_, or either diagonal _(2)_.

## 05-4:  Is there a Tie?
Complete the **is_tie()** function. It should return **true** if there is a tie on the board.  Otherwise, it should return **false**.  A **tie** occurs if the board is filled, with 'X's and 'O's but there is **no win**.
