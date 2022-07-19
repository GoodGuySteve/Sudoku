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
			[5, 6, 0, 8, 9, 1, 2, 3, 4],
			[8, 9, 1, 2, 3, 4, 5, 6, 7],
			[3, 4, 5, 6, 7, 8, 9, 1, 2],
			[6, 7, 8, 9, 1, 2, 3, 4, 5],
			[9, 1, 2, 3, 4, 5, 6, 7, 8]];

	let mut test_board = Board::new_from_array(&test_array);
	println!("board before solve: ");
	test_board.print();
	println!("-----------------------------");
	crate::wave_function_solver::solve(&mut test_board);
	test_board.print();

    println!("board is valid: {}, is solved:  {}", 
		test_board.is_valid(), 
		test_board.is_solved()); 
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
	
		let empty_board = Board::new_from_array(&empty_array);
	
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

		let test_board = Board::new_from_array(&test_array);

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

		let mut test_board = Board::new_from_array(&near_complete_array);

		assert_eq!(true, test_board.is_valid());
		assert_eq!(false, test_board.is_solved());

		assert_eq!(true, crate::wave_function_solver::solve(&mut test_board));
		assert_eq!(true, test_board.is_valid());
		assert_eq!(true, test_board.is_solved());

	}

	#[test]
	fn solve_a_few_missing() {

		let test_array = 
		   [[0, 2, 3, 4, 5, 0, 7, 8, 9],
			[4, 5, 0, 7, 8, 9, 1, 2, 3],
			[7, 8, 9, 1, 2, 3, 4, 5, 6],
			[2, 3, 4, 5, 6, 7, 8, 9, 1],
			[5, 6, 7, 8, 9, 1, 2, 3, 4],
			[8, 9, 1, 2, 3, 4, 5, 6, 7],
			[3, 4, 5, 6, 7, 8, 9, 1, 2],
			[0, 7, 8, 9, 1, 0, 3, 4, 5],
			[9, 1, 2, 3, 4, 5, 6, 7, 8]];

		let mut test_board = Board::new_from_array(&test_array);

		assert_eq!(true, test_board.is_valid());
		assert_eq!(false, test_board.is_solved());

		assert_eq!(true, crate::wave_function_solver::solve(&mut test_board));
		assert_eq!(true, test_board.is_valid());
		assert_eq!(true, test_board.is_solved());
	}

	#[test]
	fn very_easy_solve() {

		let test_array = 
		   [[9, 2, 0, 0, 4, 1, 0, 5, 0],
			[1, 0, 0, 7, 0, 5, 0, 6, 3],
			[7, 0, 3, 8, 9, 0, 2, 0, 0],
			[0, 0, 0, 1, 6, 3, 8, 0, 0],
			[0, 0, 0, 0, 0, 0, 5, 4, 7],
			[2, 9, 8, 0, 0, 0, 0, 0, 1],
			[5, 4, 0, 0, 3, 7, 0, 8, 9],
			[0, 6, 0, 9, 0, 0, 4, 2, 0],
			[0, 1, 0, 0, 5, 2, 0, 7, 0]];

		let mut test_board = Board::new_from_array(&test_array);

		assert_eq!(true, test_board.is_valid());
		assert_eq!(false, test_board.is_solved());

		assert_eq!(true, crate::wave_function_solver::solve(&mut test_board));
		assert_eq!(true, test_board.is_valid());
		assert_eq!(true, test_board.is_solved());
	}
	#[test]
	fn very_easy_solve2() {

		let test_array = 
		   [[0, 0, 0, 4, 0, 5, 0, 0, 0],
			[0, 0, 9, 0, 6, 0, 0, 8, 0],
			[5, 0, 0, 0, 0, 8, 0, 7, 4],
			[0, 0, 0, 3, 9, 6, 2, 0, 0],
			[0, 3, 2, 1, 0, 7, 4, 0, 6],
			[1, 0, 6, 0, 8, 4, 0, 0, 0],
			[0, 8, 0, 0, 7, 2, 6, 3, 1],
			[0, 5, 7, 0, 3, 0, 0, 4, 0],
			[6, 0, 3, 5, 4, 0, 0, 2, 0]];

		let mut test_board = Board::new_from_array(&test_array);

		assert_eq!(true, test_board.is_valid());
		assert_eq!(false, test_board.is_solved());

		assert_eq!(true, crate::wave_function_solver::solve(&mut test_board));
		assert_eq!(true, test_board.is_valid());
		assert_eq!(true, test_board.is_solved());
	}

}