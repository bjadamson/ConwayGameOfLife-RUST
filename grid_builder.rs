extern mod extra;
use std::vec::from_elem;
use grid::{Row, Column, alive, dead, Cell, Grid};
mod grid;

/****************************************************************************
 * Something to say Ben??
 ****************************************************************************/

/*
 * Constructs an immutable grid from the contents of an array of strings.
 * The grid is declared mutable, and mutated only inside this fn.
*/
pub fn build_from_file_contents(file_contents: ~[~str]) -> Grid
{
  let height = file_contents.len();
  assert!(height > 0u);
  let width = file_contents[0].len();
  let cells = from_elem(width, Cell { value: alive } );

  let mut result = Grid {
      inner: from_elem(height, cells.clone())
  };
  for row in range(0, height) {
    for column in range(0, width) {
      let file_value: char = file_contents[row][column] as char;
      let cell_value = match file_value {
        'o' => alive,
        '.' => dead,
         _  => fail!("Unexpected cell value found in file.")
      };
      result.inner[row][column] = Cell { value: cell_value };
   }
  }
  return result;
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

/*pub fn print_neighbor_count(grid: &Grid) {
  let x = grid.inner.len();
  for row in range(0, x) {
    for column in range(0, x) {
      let r = Row(row);
      let c = Column(column);
      let n = count_neighbors(r, c, grid);
      print!("{:u}", n);
    }
    println!("");
  }
}  // fn print_neighbor_count
*/

// 1)Any live cell with fewer than two live neighbours dies, as if caused by
// under-population.
// 2) Any live cell with two or three live neighbours lives on to the next
// generation.
// 3) Any live cell with more than three live neighbours dies, as if by
// overcrowding.
// 4) Any dead cell with exactly three live neighbours becomes a live cell, as
// if by reproduction.
pub fn build_from_grid(prevg: &Grid) -> Grid
{
  let cell_value = |row: uint, column: uint| {
    let ncount = count_neighbors(Row(row), Column(column), prevg);
    let cv = match (prevg.inner[row][column].value, ncount) {
      (dead, 3)       => alive,
      (alive, 2..3)   => alive,
      _               => dead
    };
    return Cell { value: cv };
  };
  let mut result = prevg.clone();
  for row in range(0, prevg.height()) {
    for column in range(0, prevg.width()) {
      result.inner[row][column] = cell_value(row, column);
    }
  }
  return result;
}  // fn build_from_grid
