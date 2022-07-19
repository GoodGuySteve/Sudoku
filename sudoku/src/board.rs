use crate::constants;
use constants::SCALE_FACTOR;
use constants::NUMBER_LIMIT;
use constants::DEBUG;

pub fn box_num(row: usize, col: usize) -> usize {
	/* Takes the zero-indexed row and column number and returns the zero-indexed box number of the given entry. sectors
	are numbered from left to right, then top to bottom. 
	
	Example: for a 3x3, box 0 is top left, box 2 is top right, box 6 is bottom left, 
	box 8 is bottom right.
	*/
	let box_row: usize = row / SCALE_FACTOR;
	let box_col: usize = col / SCALE_FACTOR;

	return SCALE_FACTOR * box_row + box_col;
}

pub fn entry_index(row: usize, col: usize) -> usize {
	return row * NUMBER_LIMIT + col;
}

/* TODO I might not be able to pass around the pointer to this struct with Copy derived */
/* For each entry, value represents the value placed in the cell (or blank, if value is 0) */
#[derive(Debug, Copy, Clone)]
pub struct Entry {
	pub value: usize,
	pub row: usize,
	pub col: usize,
	pub sector: usize,
    pub options: [bool; NUMBER_LIMIT], // true = is possible, false = not possible for each index
}

impl Entry {
    pub fn init(&mut self, row: usize, col: usize, value: usize) {
        self.value = value;
        self.row = row;
        self.col = col;
        self.sector = box_num(row, col);
        self.options = [true; NUMBER_LIMIT];
    }

	pub fn is_valid(&self) -> bool {
		/* This is the thorough validation check to make sure all fields are in bounds. */
		if self.value > NUMBER_LIMIT {
			return false;
		}
		if self.row >= NUMBER_LIMIT {
			return false;
		}
		if self.col >= NUMBER_LIMIT {
			return false;
		}
		if self.sector >= NUMBER_LIMIT {
			return false;
		}
        if !self.has_options() {
            return false;
        }

		return true;
	}

    /* Returns true if there is still a possible option to place into the value field. Always returns
       true if the value field is specified to a valid number. */
    pub fn has_options(&self) -> bool {
        if self.value > 0 && self.value <= NUMBER_LIMIT {
            return true;
        } else {
            for option in self.options {
                /* The only invalid state is if the cell's value has not been specified
                   and there are no possible options left. */
                if option == true {
                    return true;
                }
            }
            return false;
        }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        return self.value  == other.value && 
               self.row    == other.row && 
               self.col    == other.col && 
               self.sector == other.sector;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Board {
	pub entries: [Entry; NUMBER_LIMIT * NUMBER_LIMIT],
}

impl Board {
	pub fn new_from_array(arr: &[[usize; NUMBER_LIMIT]; NUMBER_LIMIT])
	-> Board {
		/* Instantiate a new Board given a 2D array of numbers to fill with */
		let mut board = Board { 
			entries: [
				Entry { 
					value: 0, 
					row: 0, 
					col: 0, 
					sector: 0,
					options: [true; NUMBER_LIMIT]
				}; NUMBER_LIMIT * NUMBER_LIMIT
			]
		};
		
		let mut row: usize;
		let mut col: usize;
		row = 0;
		col = 0;
		for square in board.entries.iter_mut() {
			let value = arr[row][col];
			square.init(row, col, value);

			col += 1;
			if col >= NUMBER_LIMIT {
				// Crossed over to a new row
				col = 0;
				row += 1;
			}
		}
		return board;
	}
	
	pub fn is_valid(&self) -> bool {
		/* Checks if the current board is a valid Sudoku board according to Sudoku rules
		   (each number appears exactly once in each row, column, and subsquare). Unset
		   entries are considered valid. Options are not validated.
		*/ 

		/* Start with each row, column, and box empty. Iterate through each entry of the sudoku and check for collisions */
		/* Position is used to map to numeric entry value */
		let mut rows: [[bool; NUMBER_LIMIT]; NUMBER_LIMIT] = [[false; NUMBER_LIMIT]; NUMBER_LIMIT];
		let mut cols: [[bool; NUMBER_LIMIT]; NUMBER_LIMIT] = [[false; NUMBER_LIMIT]; NUMBER_LIMIT];
		let mut sectors: [[bool; NUMBER_LIMIT]; NUMBER_LIMIT] = [[false; NUMBER_LIMIT]; NUMBER_LIMIT];

		for entry in self.entries.iter() {
			if !entry.is_valid() {
				/* This is out of range, so an error is more appropriate */
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
				if sectors[entry.sector][entry.value - 1] == true {
					/* We've seen this entry in this box already */
					if DEBUG {
						println!("{}, {}: {} already present in box {}.", entry.row, entry.col, entry.value, entry.sector);
					}
					return false;
				} else {
					/* Mark that we've seen this entry in this box */
					sectors[entry.sector][entry.value - 1] = true;
				}
			}
		}

		return true;
	}

    /* Returns true if all values have been filled in and are valid */
    pub fn is_solved(&self) -> bool {
		for entry in self.entries.iter() {
			if entry.value == 0 {
				return false;
			}
        }
        return self.is_valid();
    }
	
	pub fn print(&self) {
		/* Prints out the entire board in a pretty format. */
		let mut i = 0;
		let mut row_str = String::from("");
		for square in self.entries.iter() {
			row_str.push_str(&square.value.to_string());
			row_str.push_str(", ");
			i += 1;
			
			if i >= NUMBER_LIMIT {
				// Row done, print and move on to the next one
				println!("{}", row_str);
				row_str = String::from("");
				i = 0;
			}
		}
	}
}