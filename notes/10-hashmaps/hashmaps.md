# HashMap

A [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) is a collection _(we have seen two other collections:  [arrays](notes/04-arrays/arrays.md) and [vectors](notes/05-vectors/vectors.md))_ that consists of **keys**, and **values**. Each **key** is associated with a single **value**. The association of a **key** and a **value** is called a **key-value pair**.  Unlike an **array**, the **keys** can be of any type.  Hence, whereas an **array** is an ordered collection of data, a `HashMap` is a **mapping** from **keys** to **values** - there is **no inherent order** to the data.

Below, you can see an example of how to create a `HashMap` using the **province-captial** information from Canada. Notice that, unlike the **array** and **vector** collections, you must import the `HashMap` **struct** from the standard library

```rust
use std::collections::HashMap;

fn main() {
	let mut capitals = HashMap::new();
	capitals.insert("Ontario", "Toronto");
	capitals.insert("Quebec", "Quebec City");
	capitals.insert("Nova Scotia", "Halifax");
	capitals.insert("New Brunswick", "Fredericton");
	capitals.insert("Manitoba", "Winnipeg");
	capitals.insert("British Columbia", "Victoria");
	capitals.insert("Prince Edward Island", "Charlottetown");
	capitals.insert("Saskatchewan", "Regina");
	capitals.insert("Alberta", "Edmonton");
	capitals.insert("Newfoundland and Labrador", "St. John's");
	capitals.insert("Northwest Territories", "Yellowknife");
	capitals.insert("Yukon", "Whitehorse");
	capitals.insert("Nunavut", "Iqaluit");

// Iterating over the HashMap
	for (province, capital) in &capitals {
		println!("{}: {}", province, capital);
	}
}
```
Here a new `HashMap` has been created using the `HashMap::new()` command.  New **key-value** pairs are added to the `HashMap` using the **insert()** method.  Unlike an **array** or **vector**, the **key-value** pairs are **not saved in any order**.  You can demonstrate this by running the code above multiple times. Notice how the the **key-value** pairs can be accessed using a **for-loop**.

A `HashMap` can also be created from an [array](/notes/04-arrays/arrays.md) of [tuples](/notes/09-tuples/tuples.md) using the `HashMap::from()` command. For example,

```rust
let map = HashMap::from([
        ('a', 1),
        ('b', 3),
        ('c', 3),
        ('d', 2),
        ('e', 1),
        ('f', 4),
        ('g', 2),
        ('h', 4),
        ('i', 1),
        ('j', 8),
        ('k', 5),
        ('l', 1),
        ('m', 3),
        ('n', 1),
        ('o', 1),
        ('p', 3),
        ('q', 10),
        ('r', 1),
        ('s', 1),
        ('t', 1),
        ('u', 1),
        ('v', 4),
        ('w', 4),
        ('x', 8),
        ('y', 4),
        ('z', 10),
    ]);
```
## Methods

### Accessing a value

In order to access a value, you can simply use provide the **key** to the `HashMap` as follows:

```rust
println!("\nCapital of Ontario: {}", capitals["Ontario"]);
```

You can also use the **get()** method by providing it a **key**. Because it is possible that there is no such **key**, the **get()** method returns an **Option** type. We can access and store the **value** using **pattern matching** as follows:
```rust
    if let Some(capital) = capitals.get("Nunavut") {
        println!("Capital of Nunavut: {}", capital);
    }
```

or, if we are confident that the `key` exists we can just use the `unwrap()` method as [follows](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=4db396ddcec7763e83fa2e45e0c659a9)


```rust
capital = capitals.get("Nunavut").unwrap();
println!("Capital: {}", capital);
```
### Removing an entry

To remove an entry, we use the **remove()** method by providing it a **key** as follows:
```rust
    capitals.remove("Ontario");
```
To run the code above, see the given file - **hashmaps.rs**.

## Using a HashMap as a Counter

You often use a `HashMap` to keep track of the number of occurrences of some type of data.

#### Example 1:

For [example](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f04cef9097aa5235f702ad0afc428617), if we wanted to count the number of emojis in the following string

```rust
let text = "👽😊🍟😊👽😺👽".to_string();
```

we can first create a new mutable `HashMap`,

```rust
let mut counter = HashMap::new();
```

The following code, iterates through the characters of the string **text** using a **for-loop**,

```rust
for c in text.chars() {
	if counter.contains_key(&c) {
		counter.insert(c, counter[&c] + 1);
	} else {
		counter.insert(c, 1);
	}
}
```

The **contains_key()** method is used to determine whether a **key** already exists, and if so, an identical entry is inserted with an updated **value**.  Otherwise, a **value** of 1 is inserted for this new **key**.  The `HashMap` can either be output by accessing a single **key** to output a single **value** or the entire `HashMap` using `Debug` printing as follows:

```rust
println!("👽: {}", counter[&'👽']);
println!("{:?}", counter);
```
The output will look something like
```
👽: 3
{'😺': 1, '👽': 3, '😊': 2, '🍟': 1}
```

#### Example 2:  Using the entry() method

Here is another [example](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=73b2130879738f9e8220777b267eb396), where we want to count the number of letters _(notice the use of the `to_lowercase()` method)_  in a string,

```rust
use std::collections::HashMap;

fn main() {
    let mut counter = HashMap::new();
    let text = "Welcome to Albert College!";

    for c in text.to_lowercase().chars() {
        let count = text.entry(c).or_insert(0);
        *count += 1;
    }

    println!("{:?}", counter);
}
```

producing the output,

```
{'w': 1, 'e': 5, 'o': 3, 't': 2, 'r': 1, 'g': 1, 'l': 4, 'm': 1, ' ': 3, 'a': 1, '!': 1, 'b': 1, 'c': 2}
```

Notice the use of the `entry()` method.  The `entry()` method in Rust's `HashMap` is a powerful tool for **modifying the value of a key** or **inserting a new key-value** pair if the key does not exist. It provides more control over how you interact with the `HashMap`, especially useful for tasks like updating counters or conditional insertion.

The `entry()` method returns an `Entry` [enum](/notes/13-enums/enums.md), which represents either an occupied entry (`OccupiedEntry`) or a vacant entry (`VacantEntry`). You can then use methods like `or_insert()`, `or_insert_with()`, or manipulate the entry directly.  In the  [example](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=73b2130879738f9e8220777b267eb396) above,

- `entry(c)`: This creates an entry for the letter _(char)_ in the `HashMap`. If the letter exists, it returns an `OccupiedEntry`; if it doesn't, it returns a `VacantEntry`.
    
- `or_insert(0)`: If the entry is vacant (i.e., the letter is not yet in the `HashMap`), this inserts the letter with a value of `0` and returns a **mutable reference to the value** called `count`. If the entry is occupied, it returns a **mutable reference to the existing value** called `count`.
    
- `*count += 1`: This increments the value for the letter, effectively counting its occurrences. Since `count` is a **mutable reference** to the value, it must be **dereferenced** using the `*` before its value can be updated.