const SCALE_FACTOR: usize = 3;

#[derive(Copy, Clone)]
struct Entry {
	value: u32,
}

struct Board {
	entries: [[Entry; SCALE_FACTOR * SCALE_FACTOR]; SCALE_FACTOR * SCALE_FACTOR],
}

impl Board {
	fn new_from_array(arr: [[u32; SCALE_FACTOR * SCALE_FACTOR]; SCALE_FACTOR * SCALE_FACTOR])
	-> Board {
		Board { entries: [[Entry { value: 0 }; SCALE_FACTOR * SCALE_FACTOR]; SCALE_FACTOR * SCALE_FACTOR]}
	}
	
	fn print(self) {
		
	}
}

fn main() {
	let test_array = [[1, 2, 3, 4, 5, 6, 7, 8, 9],
	    [4, 5, 6, 7, 8, 8, 1, 2, 3],
		[7, 8, 9, 1, 2, 3, 4, 5, 6],
		[2, 3, 4, 5, 6, 7, 8, 9, 1],
		[5, 6, 7, 8, 9, 1, 2, 3, 4],
		[8, 9, 1, 2, 3, 4, 5, 6, 7],
		[3, 4, 5, 6, 7, 8, 9, 1, 2],
		[6, 7, 8, 9, 1, 2, 3, 4, 5],
		[9, 1, 2, 3, 4, 5, 6, 7, 8]];

	let test_board = Board::new_from_array(test_array);
	test_board.print()

    println!("Hello, world!");
}
