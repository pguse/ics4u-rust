fn main() {

	let marks = [75, 82, 90, 95, 87, 80, 70, 92];
	let mut sum = 0;

	// Simple array output - You can comment this out
	println!("{:?}", marks);

	for _i in 0..marks.len() {
		// Update the accumulator
		sum = marks[0];
	}

	print!("Marks:  ");
	// Insert a Loop
	print!("{} ", marks[0]);

	// Correct this calculation
	let average = sum as f64;
	println!("\nAverage:  {:0.1}", average);
}