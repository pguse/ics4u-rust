fn main() {
    let m = [ [1, 2, 3],
              [3, 4, 5],
              [6, 7, 8]];
    println!("Total: {}", total(&m));
}

fn total(a: &[[i32;3]]) -> i32 {
	let mut s = 0;
	for row in 0..3 {
		for col in 0..3 {
			s += a[row][col];
		}
	}
    s
}