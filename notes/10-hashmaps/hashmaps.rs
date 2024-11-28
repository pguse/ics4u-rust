use std::collections::HashMap;

fn main() {
    // Creating a HashMap
    let mut capitals = HashMap::new();
    // Inserting key-value pairs into the HashMap
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

    // Accessing a value using a key
    println!("\nCapital of Ontario: {}", capitals["Ontario"]);

    // Accessing a value using a key with pattern matching
    if let Some(capital) = capitals.get("Nunavut") {
        println!("\nCapital of Nunavut: {}", capital);
    }

	capitals.remove("Ontario");

	println!("\n{:?}", capitals);
}