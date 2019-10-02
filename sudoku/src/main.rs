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
		/* Instantiate a new Board given a 2D array of numbers to fill with */
		let mut board = Board { entries: [[Entry { value: 0 }; SCALE_FACTOR * SCALE_FACTOR]; SCALE_FACTOR * SCALE_FACTOR]};
		
		let mut i: usize;
		let mut j: usize;
		i = 0;
		for row in board.entries.iter_mut() {
			j = 0;
			for square in row.iter_mut() {
				square.value = arr[i][j];
				j += 1;
			}
			i += 1;
		}
		return board;
	}
	
	fn print(self) {
		/* Prints out the entire board in a pretty format. */
		/* Note that this print statement is probably inefficient. Consider preallocation if 
		   this is slow. */
		for row in self.entries.iter() {
			let mut row_str = String::from("");
			for square in row.iter() {
				row_str.push_str(&square.value.to_string());
				row_str.push_str(", ")
			}
			println!("{}", row_str);
		}
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
	test_board.print();

    println!("Hello, world!");
}
