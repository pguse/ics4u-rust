fn main() {
	let board = ['X', 'O', 'X', 'O', 'X', '-', '-', 'X', 'O'];

	display(board)
}

fn display(b: [char; 9]) {
	for i in 0..b.len() {
        if i % 3 == 0{
            print!("\n{}  ", b[i]);
        } else {
            print!("{}  ", b[i]);
        }
	}
}

fn is_win(b: [char;9]) -> bool {
	false
}

fn is_tie(b: [char;9]) -> bool {
	false
}