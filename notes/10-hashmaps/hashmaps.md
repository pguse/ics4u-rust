# HashMap

A `HashMap` is a collection _(we have seen two other collections:  [arrays](notes/04-arrays/arrays.md) and [vectors](notes/05-vectors/vectors.md))_ that consists of **keys**, and **values**. Each **key** is associated with a single **value**. The association of a **key** and a **value** is called a **key-value pair**.  Unlike an **array**, the **keys** can be of any type.  Hence, whereas an **array** is an ordered collection of data, a `HashMap` is a **mapping** from **keys** to **values** - there is **no inherent order** to the data.

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
Here a new `HashMap` has been created using the **HashMap::new()** command.  New **key-value** pairs are added to the `HashMap` using the **insert()** method.  Unlike an **array** or **vector**, the **key-value** pairs are **not saved in any order**.  You can demonstrate this by running the code above multiple times. Notice how the the **key-value** pairs can be accessed using a **for-loop**.
## Accessing a value

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
## Removing an entry

To remove an entry, we use the **remove()** method by providing it a **key** as follows:
```rust
    capitals.remove("Ontario");
```
To run the code above, see the given file - **hashmaps.rs**.

## Using a HashMap as a Counter

You often use a `HashMap` to keep track of the number of occurrences of some type of data.  For example ***(see the given file - hashmaps1.rs)***, if we wanted to count the number of emojis in the following string

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