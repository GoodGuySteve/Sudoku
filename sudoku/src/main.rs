mod constants;

mod board;
use board::Board;

mod wave_function_solver;

fn main() {
	let test_array = 
	   [[1, 2, 3, 4, 5, 6, 7, 8, 9],
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

    println!("test_board is valid: {}", test_board.is_valid());

	let empty_array = 
	   [[0, 0, 0, 0, 0, 0, 0, 0, 0],
	    [0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0]];

	let empty_board = Board::new_from_array(empty_array);
	empty_board.print();

    println!("empty_board is valid: {}", empty_board.is_valid());

	let near_complete_array = 
	   [[1, 2, 3, 4, 5, 6, 7, 8, 9],
	    [4, 5, 6, 7, 8, 9, 1, 2, 3],
		[7, 8, 9, 1, 2, 3, 4, 5, 6],
		[2, 3, 4, 5, 6, 7, 8, 9, 1],
		[5, 6, 0, 8, 9, 1, 2, 3, 4],
		[8, 9, 1, 2, 3, 4, 5, 6, 7],
		[3, 4, 5, 6, 7, 8, 9, 1, 2],
		[6, 7, 8, 9, 1, 2, 3, 4, 5],
		[9, 1, 2, 3, 4, 5, 6, 7, 8]];

	let mut near_complete_board = Board::new_from_array(near_complete_array);
	crate::wave_function_solver::solve(&mut near_complete_board);
	near_complete_board.print();

    println!("near_complete_board is valid: {}, is solved:  {}", 
		near_complete_board.is_valid(), 
		near_complete_board.is_solved());
}

#[cfg(test)]
mod tests {
	use crate::board::Board;

	#[test]
	fn validate_empty() {
		let empty_array = 
		   [[0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0]];
	
		let empty_board = Board::new_from_array(empty_array);
	
		assert_eq!(true, empty_board.is_valid());
		assert_eq!(false, empty_board.is_solved());
	}

	#[test]
	fn validate_full() {
			
		let test_array = 
		[[1, 2, 3, 4, 5, 6, 7, 8, 9],
		[4, 5, 6, 7, 8, 9, 1, 2, 3],
		[7, 8, 9, 1, 2, 3, 4, 5, 6],
		[2, 3, 4, 5, 6, 7, 8, 9, 1],
		[5, 6, 7, 8, 9, 1, 2, 3, 4],
		[8, 9, 1, 2, 3, 4, 5, 6, 7],
		[3, 4, 5, 6, 7, 8, 9, 1, 2],
		[6, 7, 8, 9, 1, 2, 3, 4, 5],
		[9, 1, 2, 3, 4, 5, 6, 7, 8]];

		let test_board = Board::new_from_array(test_array);

		assert_eq!(true, test_board.is_valid());
		assert_eq!(true, test_board.is_solved());
	}

	#[test]
	fn simple_solver() {

		let near_complete_array = 
		   [[1, 2, 3, 4, 5, 6, 7, 8, 9],
			[4, 5, 6, 7, 8, 9, 1, 2, 3],
			[7, 8, 9, 1, 2, 3, 4, 5, 6],
			[2, 3, 4, 5, 6, 7, 8, 9, 1],
			[5, 6, 0, 8, 9, 1, 2, 3, 4],
			[8, 9, 1, 2, 3, 4, 5, 6, 7],
			[3, 4, 5, 6, 7, 8, 9, 1, 2],
			[6, 7, 8, 9, 1, 2, 3, 4, 5],
			[9, 1, 2, 3, 4, 5, 6, 7, 8]];

		let mut test_board = Board::new_from_array(near_complete_array);

		assert_eq!(true, test_board.is_valid());
		assert_eq!(false, test_board.is_solved());

		assert_eq!(true, crate::wave_function_solver::solve(&mut test_board));
		assert_eq!(true, test_board.is_valid());
		assert_eq!(true, test_board.is_solved());

		// TODO solve and revalidate

	}
}