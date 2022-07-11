/* Solves via "wave function collapse" methodology. The methodology goes as follows:
   1) Input a value for one of the entries with the lowest number of options. If
      there is more than one option, 'randomly' guess.
   2) Collapse all options based on the new guess.
   3) Repeat until the board is either solved or invalidated.
   4) If board is invalidated, back-track to the last state that still has available
      options to guess from
*/
//use crate::constants;
//use constants::DEBUG;

use crate::board::Board;
use crate::board::Entry;

/* Takes a board and removes all options that are invalidated by basic Sudoku rules
   (meaning those options that already exist in their row, column, or sector) */
pub fn collapse_options(board: &mut Board) {
    for board_line in board.entries {
        for mut entry in board_line {
            for (optionIndex, isOption) in entry.options.iter().enumerate() {
                /* Options is an array of booleans, with each index corresponding to 
                   a value. Since Sudoku is 0-indexed, we need to add 1 */
                if *isOption && !can_entry_have_value(*board, entry, optionIndex + 1) {
                    entry.options[optionIndex] = false;
                }
            }
        }
    }
}

/* Returns true if putting the suggested value into the given entry of the board
   does not violate Sudoku constraints. */
fn can_entry_have_value(board: Board, entry: Entry, value: usize) -> bool {
    
    let row = entry.row;
    let col = entry.col;
    let sector = entry.sector;

    for board_line in board.entries {
        for e in board_line {
            if e.row == row || e.col == col || e.sector == sector {
                if e.value == value {
                    // This value already exists in the row/column/sector, so reject
                    // (unless we just compared the same object)
                    if e != entry {
                        return false;
                    }
                }
            }
        }
    }
    return true;
}