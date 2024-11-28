use std::collections::HashMap;

fn main() {
    let text =  "👽😊🍟😊👽😺👽".to_string();

    let mut counter = HashMap::new();

    for c in text.chars() {
        if counter.contains_key(&c) {
            counter.insert(c, counter[&c] + 1);
        } else {
            counter.insert(c, 1);
        }
    }

    println!("👽: {}", counter[&'👽']);
    println!("{:?}", counter);
}