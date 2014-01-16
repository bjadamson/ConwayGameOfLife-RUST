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

enum direction {
  above, below, left, right
}  // enum CellValue

/*
  Calculates the neighbor's position given the inputs.
*/
fn neighbor_pos(Row(row): Row, Column(column): Column, dir: direction)
    -> (Row, Column)
{
  return match dir {
    above => (Row(row - 1), Column(column)),
    below => (Row(row + 1), Column(column)),
    right => (Row(row), Column(column + 1)),
    left  => (Row(row), Column(column - 1))
  };
}  // fn neighbor_pos

/*
 * Returns a count for how many neighbors of a cell in the grid
 * are alive.
*/
pub fn count_neighbors(row: Row, column: Column, grid: &Grid)
    -> uint
{
  let (same_row, left_column)  = neighbor_pos(row, column, left);
  let (same_row, right_column) = neighbor_pos(row, column, right);
  let (above_row, same_column) = neighbor_pos(row, column, above);
  let (below_row, same_column) = neighbor_pos(row, column, below);

  let (above_left_row, above_left_column) = neighbor_pos(above_row, same_column, left);
  let (above_right_row, above_right_column) = neighbor_pos(above_row, same_column, right);
  let (below_left_row, below_left_column) = neighbor_pos(below_row, same_column, left);
  let (below_right_row, below_right_column) = neighbor_pos(below_row, same_column, right);

  let c0 = grid.cell_alive(same_row, left_column);
  let c1 = grid.cell_alive(same_row, right_column);
  let c2 = grid.cell_alive(above_row, same_column);
  let c3 = grid.cell_alive(below_row, same_column);

  let c4 = grid.cell_alive(above_left_row, above_left_column);
  let c5 = grid.cell_alive(above_right_row, above_right_column);
  let c6 = grid.cell_alive(below_left_row, below_left_column);
  let c7 = grid.cell_alive(below_right_row, below_right_column);
  return c0 + c1 + c2 + c3 + c4 + c5 + c6 + c7;
}

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
