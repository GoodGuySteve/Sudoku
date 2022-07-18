/* Solves via "wave function collapse" methodology. The methodology goes as follows:
   1) Input a value for one of the entries with the lowest number of options. If
      there is more than one option, 'randomly' guess.
   2) Collapse all options based on the new guess.
   3) Repeat until the board is either solved or invalidated.
   4) If board is invalidated, back-track to the last state that still has available
      options to guess from
*/
use crate::constants::NUMBER_LIMIT;
//use constants::DEBUG;

use crate::board::Board;
use crate::board::Entry;

pub fn solve(board: &mut Board) {
    // Start by making sure all the options are resolved according to sudoku rules
    collapse_options(board);

    // TODO guess, collapse, and repeat
    // TODO sort for smallest number of options
    // TODO push old state onto a stack
    // TODO make guess
    //   TODO options of 0 means broke and backtrack, no options at all means solved
    // TODO collapse options and repeat
}

/* Takes a board and removes all options that are invalidated by basic Sudoku rules
   (meaning those options that already exist in their row, column, or sector) */
pub fn collapse_options(board: &mut Board) {
    /*for board_line in board.entries.iter_mut() {
        for entry in board_line.iter_mut() {
            for option_index in 0..(entry.options.len() - 1) {
                /* Options is an array of booleans, with each index corresponding to 
                   a value. Since Sudoku is 0-indexed, we need to add 1 */
                if entry.options[option_index] && !can_entry_have_value(board, entry, option_index + 1) {
                    entry.options[option_index] = false;
                }
            }
        }
    }*/
    for row in 0..(NUMBER_LIMIT - 1) {
        for col in 0..(NUMBER_LIMIT - 1) {
            let entry = &board.entries[col*NUMBER_LIMIT + row];
            for option_index in 0..(entry.options.len() - 1) {
                /* Options is an array of booleans, with each index corresponding to 
                   a value. Since Sudoku is 0-indexed, we need to add 1 */
                if entry.options[option_index] && !can_entry_have_value(board, entry, option_index + 1) {
                    let mut options = board.entries[col*NUMBER_LIMIT + row].options;
                    options[option_index] = false;
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