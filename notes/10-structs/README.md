# Structs

At this point we have used functions to organize code and primitive data types to organize data.  Now, we are going to build our own custom types to organize both code and data.

## Custom Types

We are going to build a **custom type** that describes student. In Rust, a custom type is called a **struct**.  A struct is made up of **fields** that may contain different types.  A **struct** definition for a **Student** would look like this

```rust
struct Student {
    name: String,
    grade: u8,
}
```

Struct definitions list the set of attributes, and we can then fill in a struct with values to represent a particular **instance**.  To instantiate a particular point and store the data in a variable, we set the variable equal to the struct name, then open curly brackets, then list each field and the value we want each field to have. 

```rust
struct Student {
    name: String,
    grade: u8,
}

fn main() {
    // An instance of Student
    let s1 = Student {
        name: String::from("Guido van Rossum"),
        grade: 11,
    };
}
```

To access individual fields of a **struct** you have to use **dot notation** on the **instance** of the struct along with the **field** name.  For example,

```rust
println!("{} is in grade {}.", s1.name, s1.grade);
```

produces the output,

```
Guido van Rossum is in grade 11.
```

### Note
The convention in Rust is that **struct** names use a format called **camel case** (e.g. Student, MyClub,etc.) whereas **fields** of structs use a format called **snake case** (eg. first_name).

## Tuple Structs

Having **named fields** is what most structs look like, but there are also **tuple structs**. Tuple structs are structs that have a name for the whole type but don't name their fields. They behave similarly to tuples. You access their fields with a dot and an index, starting with zero. 

```rust
struct Point(f64,f64);
```

If you want to use a function with a **struct** type, it can be used like any other type.  For example, the function **midpoint**

```rust
fn midpoint(p1: Point, p2: Point) -> Point {
	Point((p1.0 + p2.0) / 2, (p1.1 + p2.1) / 2)
}
```

takes two parameters of type **Point** and returns a value of type **Point**.  The following code,

```rust
#[derive(Debug)]
struct Point(f64, f64);

fn main() {
    let p1 = Point(4.0, 5.0);
    let p2 = Point(1.0, 5.0);
    println!("Midpoint: {:?}", midpoint(p1, p2));
}

fn midpoint(p1: Point, p2: Point) -> Point {
	Point((p1.0 + p2.0) / 2.0, (p1.1 + p2.1) / 2.0)
}
```

produces the output,

```
Midpoint: Point(2.5, 5.0)
```

**Notice:**  We have not talked about **traits** in **Rust**.  In order to output our **Point** data type, you must implement the **Debug** trait for **Point**.  We do that by adding the attribute ```#[derive(Debug)]``` to **Point**.  This allows us to output our **Point** data type in a similar way to how we output an **array** using ```{:?}``` within the **println!** macro.

## Types and Methods

Using a **struct** you can create a custom type.  Like other modern languages **Rust** allows you to attach **methods** to these types.  We define the data that makes up custom types using **structs**.  **Methods** are how we define **behavior** on custom types. For example, to declare a **Fraction** type, we could used the following code,

```rust
struct Fraction(i32, i32);
```

If we wanted to multiply two **Fraction** types, we could just create a function, as follows:

```rust
#[derive(Debug)]
struct Fraction(i32, i32);

fn main() {
    let f1 = Fraction(2, 3);
    let f2 = Fraction(-1, 5);

    println!("{:?} x {:?} = {:?}", f1, f2, mult(&f1,&f2));
}

fn mult(f_1: &Fraction, f_2: &Fraction) -> Fraction {
    Fraction{num: f_1.0 * f_2.0, den: f_1.1 * f_2.1}
}
```

produces the output,

```
Fraction(2, 3) x Fraction(-1, 5) = Fraction(-2, 15)
```

### Methods

Why do we create methods?  When a struct is combined with methods both the data and the associated actions on that data are grouped together.  As programs become larger this is the preferred way of organizing them.

How do we define a method? Methods are defined within a block
that starts with the impl keyword, which is short for implementation. Then we specify what we're implementing methods on, which, in this case, is the **Student** struct.   For example,

```rust
#[derive(Debug)]
struct Fraction(i32, i32);

impl Fraction {
    fn mult(self, f: &Fraction) -> Fraction {
        Fraction(self.0 * f.0, self.1 * f.1)
    }
}
```

This method can be used as follows,

```rust
fn main() {
    let f1 = Fraction(2, 3);
    let f2 = Fraction(-1, 5);

    println!("{:?}", f1.mult(&f2));
}
```

by using the **dot notation**.  This gives the output,

```
Fraction(-2, 15)
```