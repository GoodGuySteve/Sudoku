const SCALE_FACTOR: usize = 3;
const NUMBER_LIMIT: usize = SCALE_FACTOR * SCALE_FACTOR;
const DEBUG: bool = true;

fn box_num(row: usize, col: usize) -> usize {
	/* Takes the zero-indexed row and column number and returns the zero-indexed box number of the given entry. Boxes
	are numbered from left to right, then top to bottom. 
	
	Example: for a 3x3, box 0 is top left, box 2 is top right, box 6 is bottom left, 
	box 8 is bottom right.
	*/
	let box_row: usize = row / SCALE_FACTOR;
	let box_col: usize = col / SCALE_FACTOR;

	return SCALE_FACTOR * box_row + box_col;
}

#[derive(Copy, Clone)]
struct Entry {
	value: usize,
	row: usize,
	col: usize,
	boxx: usize,
}

struct Board {
	entries: [[Entry; NUMBER_LIMIT]; NUMBER_LIMIT],
}

impl Entry {

}

impl Board {
	fn new_from_array(arr: [[usize; NUMBER_LIMIT]; NUMBER_LIMIT])
	-> Board {
		/* Instantiate a new Board given a 2D array of numbers to fill with */
		let mut board = Board { 
			entries: [
				[
					Entry { value: 0, row: 0, col: 0, boxx: 0 }; NUMBER_LIMIT
				]; NUMBER_LIMIT
			]
		};
		
		let mut i: usize;
		let mut j: usize;
		i = 0;
		for row in board.entries.iter_mut() {
			j = 0;
			for square in row.iter_mut() {
				square.value = arr[i][j];
				square.row = i;
				square.col = j;
				square.boxx = box_num(i, j);
				j += 1;
			}
			i += 1;
		}
		return board;
	}
	
	fn is_valid(&self) -> bool {
		/* Checks if the current board is a valid Sudoku board according to Sudoku rules
		   (each number appears exactly once in each row, column, and subsquare). Unset
		   entries are considered valid.
		*/ 

		/* Start with each row, column, and box empty. Iterate through each entry of the sudoku and check for collisions */
		/* Position is used to map to numeric entry value */
		let mut rows: [[bool; NUMBER_LIMIT]; NUMBER_LIMIT] = [[false; NUMBER_LIMIT]; NUMBER_LIMIT];
		let mut cols: [[bool; NUMBER_LIMIT]; NUMBER_LIMIT] = [[false; NUMBER_LIMIT]; NUMBER_LIMIT];
		let mut boxes: [[bool; NUMBER_LIMIT]; NUMBER_LIMIT] = [[false; NUMBER_LIMIT]; NUMBER_LIMIT];

		for row in self.entries.iter() {
			for entry in row.iter() {
				if (entry.value - 1) > NUMBER_LIMIT {
					/* This is out of range, so an error is more appropriate */
					/* TODO we should make a function to validate all of the entry fields. Just in case.*/
					println!("{}, {} contains invalid entry.", entry.row, entry.col);
					return false;
				} else if entry.value != 0 {
					/* 0 counts as empty */

					if rows[entry.row][entry.value - 1] == true {
						/* We've seen this entry in this row already */
						if DEBUG {
							println!("{}, {}: {} already present in row {}.", entry.row, entry.col, entry.value, entry.row);
						}
						return false;
					} else {
						/* Mark that we've seen this entry in this row */
						rows[entry.row][entry.value - 1] = true;
					}
					if cols[entry.col][entry.value  - 1] == true {
						/* We've seen this entry in this column already */
						if DEBUG {
							println!("{}, {}: {} already present in column {}.", entry.row, entry.col, entry.value, entry.col);
						}
						return false;
					} else {
						/* Mark that we've seen this entry in this column */
						cols[entry.col][entry.value - 1] = true;
					}
					if boxes[entry.boxx][entry.value - 1] == true {
						/* We've seen this entry in this box already */
						if DEBUG {
							println!("{}, {}: {} already present in box {}.", entry.row, entry.col, entry.value, entry.boxx);
						}
						return false;
					} else {
						/* Mark that we've seen this entry in this box */
						boxes[entry.boxx][entry.value - 1] = true;
					}
				}

			}
		}

		return true;
	}
	
	fn print(&self) {
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
	    [4, 5, 6, 7, 8, 9, 1, 2, 3],
		[7, 8, 9, 1, 2, 3, 4, 5, 6],
		[2, 3, 4, 5, 6, 7, 8, 9, 1],
		[5, 6, 7, 8, 9, 1, 2, 3, 4],
		[8, 9, 1, 2, 3, 4, 5, 6, 7],
		[3, 4, 5, 6, 7, 8, 9, 1, 2],
		[6, 7, 8, 9, 1, 2, 3, 4, 5],
		[9, 1, 2, 3, 4, 5, 6, 7, 8]];

	let test_board = Board::new_from_array(test_array);
	test_board.print();

    println!("{}", test_board.is_valid());
}
