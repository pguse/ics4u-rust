use rand::Rng;

fn main() {
	let mut n = [[0;3];3];

	// Create a random 2D array with values 1->9
	for row in 0..3 {
		for col in 0..3 {
			n[row][col] = rand::thread_rng().gen_range(1..=9);
		}
	}

	println!("{:?}", n);
	println!("{:?}", init_array());
}

// Return a random 2D array with values 1->9
fn init_array() -> [[i32;3]; 3] {
	let mut a = [[0;3];3];
	for row in 0..3 {
		for col in 0..3 {
			a[row][col] = rand::thread_rng().gen_range(1..=9);
		}
	}
	a
}