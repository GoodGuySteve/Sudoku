/* Solves via "wave function collapse" methodology. The methodology goes as follows:
   1) Input a value for one of the entries with the lowest number of options. If
      there is more than one option, 'randomly' guess.
   2) Collapse all options based on the new guess.
   3) Repeat until the board is either solved or invalidated.
   4) If board is invalidated, back-track to the last state that still has available
      options to guess from
*/
use crate::constants::NUMBER_LIMIT;
use crate::constants::DEBUG_SOLVER_STEPS;

use crate::board::Board;
use crate::board::Entry;
use crate::board::entry_index;

// Attempts to solve a sudoku board recursively. Returns true on success, false on failure.
pub fn solve(board: &mut Board) -> bool {
    // Start by making sure all the options are resolved according to sudoku rules
    collapse_options(board);

    // TODO guess, collapse, and repeat
    // TODO sort for smallest number of options

    // First, create an array of the unfilled entries
    //let mut entriesAsVec: Vec<Entry> = board.entries.iter().cloned().collect();
    let entries_as_vec_ref: Vec<&Entry> = board.entries.iter().collect();
    let mut unfilled_entries: Vec<&Entry> = entries_as_vec_ref.iter().cloned().filter(|e| e.value == 0).collect();
    if unfilled_entries.len() == 0 {
        // All entries have been filled - the puzzle is solved.
        return true;
    }

    // Sort all entries by how many options they have available. We want to pick the 
    // one with the least options.
    unfilled_entries.sort_by(|a, b| a.options.iter().filter(|c| **c == true).count().cmp(
                           &b.options.iter().filter(|c| **c == true).count()));
    let guess = unfilled_entries[0];

    // For simplicity, we just find the first of the options that is possible and guess it.
    let mut guess_value = 0;
    for i in 0..NUMBER_LIMIT {
        if guess.options[i] == true {
            guess_value = i + 1;

            // Now try putting the guess in the cell and solving.
            let mut new_board = board.clone();
            new_board.entries[entry_index(guess.row, guess.col)].value = guess_value;

            if DEBUG_SOLVER_STEPS {
                println!("Trying guess of value {} at row {}, col {}", guess_value, guess.row, guess.col);
                new_board.print();
            }

            if solve(&mut new_board) {
                // Our guess was successful, so pass it up the chain
                *board = new_board;
                return true;
            }
        }
    }
    if guess_value == 0 {
        // This cell has no valid options to fill it, therefore the sudoku is unsolvable.
        return false;
    }

    return false;
}

/* Takes a board and removes all options that are invalidated by basic Sudoku rules
   (meaning those options that already exist in their row, column, or sector) */
pub fn collapse_options(board: &mut Board) {
    for row in 0..NUMBER_LIMIT {
        for col in 0..NUMBER_LIMIT {
            for option_index in 0..NUMBER_LIMIT {
                let entry = &board.entries[entry_index(row, col)];
                /* Options is an array of booleans, with each index corresponding to 
                   a value. Since Sudoku is 0-indexed, we need to add 1 */
                if entry.options[option_index] && !can_entry_have_value(board, entry, option_index + 1) {
                    let options = &mut board.entries[entry_index(row, col)].options;
                    (*options)[option_index] = false;
                }
            }
        }
    }
}

/* Returns true if putting the suggested value into the given entry of the board
   does not violate Sudoku constraints. */
fn can_entry_have_value(board: &Board, entry: &Entry, value: usize) -> bool {
    
    let row = entry.row;
    let col = entry.col;
    let sector = entry.sector;

    for e in board.entries {
        if e.row == row || e.col == col || e.sector == sector {
            if e.value == value {
                // This value already exists in the row/column/sector, so reject
                // (unless we just compared the same object)
                if e != *entry {
                    return false;
                }
            }
        }
    }
    return true;
}