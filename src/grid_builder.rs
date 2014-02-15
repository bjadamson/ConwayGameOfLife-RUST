extern mod extra;
extern mod std;
use grid::{Row, Column, alive, dead, Cell, Grid};
mod grid;

/****************************************************************************
 * Something to say Ben??
 ****************************************************************************/

/*
 * Constructs an immutable grid from the contents of an array of strings.
*/
pub fn build_from_file_contents(file_contents: ~[~str]) -> Grid {
  let height = file_contents.len();
  assert!(height > 0u);
  let width = file_contents[0].len();
  assert!(width > 0u);
  let cell_value = |file_value| {
    return match file_value {
      'o' => alive,
      '.' => dead,
       _  => fail!("Unexpected cell value found in file.")
    };
  };
  return Grid {
    inner: std::vec::from_fn(height, |row| {
      std::vec::from_fn(width, |column| {
        assert_eq!(width, file_contents[row].len());
        let file_value = file_contents[row][column];
        return Cell { value: cell_value(file_value as char) };
      })
    })
  };
} // fn build_from_file_contents

/*
 Returns a count for how many neighbors of a cell in the grid
 are alive.
 Starts to the cell to the left of the cell/row and sums up cell_alive
 working in clockwise order.
*/
fn count_neighbors(Row(row): Row, Column(col): Column, grid: &Grid) -> uint {
  let left_column  = Column(col - 1);
  let right_column = Column(col + 1);
  let above_row    = Row(row - 1);
  let below_row    = Row(row + 1);

  return grid.cell_alive(Row(row), left_column) + // left
      grid.cell_alive(above_row, left_column)   + // left-above
      grid.cell_alive(above_row, Column(col))   + // above
      grid.cell_alive(above_row, right_column)  + // above-right
      grid.cell_alive(Row(row), right_column)   + // right
      grid.cell_alive(below_row, right_column)  + // below-right
      grid.cell_alive(below_row, Column(col))   + // below
      grid.cell_alive(below_row, left_column);    // below-left
}  // fn count_neighbors

// 1)Any live cell with fewer than two live neighbours dies, as if caused by
// under-population.
// 2) Any live cell with two or three live neighbours lives on to the next
// generation.
// 3) Any live cell with more than three live neighbours dies, as if by
// overcrowding.
// 4) Any dead cell with exactly three live neighbours becomes a live cell, as
// if by reproduction.
pub fn build_from_grid(prevg: &Grid) -> Grid {
  let cell_value = |row: uint, column: uint| {
    let ncount = count_neighbors(Row(row), Column(column), prevg);
    let cv = match (prevg.inner[row][column].value, ncount) {
      (dead, 3)       => alive,
      (alive, 2..3)   => alive,
      _               => dead
    };
    return Cell { value: cv };
  };
  return Grid {
    inner: std::vec::from_fn(prevg.height(), |row| {
      std::vec::from_fn(prevg.width(), |column| {
        cell_value(row, column)
      })
    })
  };
}  // fn build_from_grid
