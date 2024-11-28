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