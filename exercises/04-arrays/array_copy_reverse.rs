fn main() {

	let values = [2, 8, 5, 10];
	let mut reverse_values = [0; 4];

    
	for i in 0..values.len() {
		reverse_values[i] = values[i];
	}

	// Insert Loop
	print!("{} ", reverse_values[0]);
}