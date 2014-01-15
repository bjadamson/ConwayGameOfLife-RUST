extern mod extra;
use std::vec::from_elem;
mod grid;

/****************************************************************************
 * Something to say Ben??
 ****************************************************************************/

/*
 * Constructs an immutable grid from the contents of an array of strings.
 * The grid is declared mutable, and mutated only inside this fn.
*/
pub fn build_from_file_contents(file_contents: ~[~str]) -> grid::Grid
{
  let height = file_contents.len();
  assert!(height > 0u);
  let width = file_contents[0].len();
  let cells = from_elem(width, grid::Cell { value: grid::alive } );

  let mut result = grid::Grid {
      inner: from_elem(height, cells.clone())
  };
  for row in range(0, height) {
    for column in range(0, width) {
      let file_value: char = file_contents[row][column] as char;
      let cell_value = match file_value {
        'o' => grid::alive,
        '.' => grid::dead,
         _  => fail!("Unexpected cell value found in file.")
      };
      result.inner[row][column] = grid::Cell { value: cell_value };
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
fn neighbor_pos(grid::Row(row): grid::Row, grid::Column(column): grid::Column, dir: direction,
    grid: &grid::Grid) -> (grid::Row, grid::Column)
{
  return match dir {
    above => (grid::Row(row - 1), grid::Column(column)),
    below => (grid::Row(row + 1), grid::Column(column)),
    right => (grid::Row(row), grid::Column(column + 1)),
    left  => (grid::Row(row), grid::Column(column - 1))
  };
}  // fn neighbor_pos

/*
 * Returns a count for how many neighbors of a cell in the grid
 * are alive.
*/
pub fn count_neighbors(row: grid::Row, column: grid::Column, grid: &grid::Grid)
    -> uint
{
  let (same_row, left_column)  = neighbor_pos(row, column, left, grid);
  let (same_row, right_column) = neighbor_pos(row, column, right, grid);
  let (above_row, same_column) = neighbor_pos(row, column, above, grid);
  let (below_row, same_column) = neighbor_pos(row, column, below, grid);

  let (above_left_row, above_left_column) = neighbor_pos(above_row, same_column, left, grid);
  let (above_right_row, above_right_column) = neighbor_pos(above_row, same_column, right, grid);
  let (below_left_row, below_left_column) = neighbor_pos(below_row, same_column, left, grid);
  let (below_right_row, below_right_column) = neighbor_pos(below_row, same_column, right, grid);

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

/*pub fn print_neighbor_count(grid: &grid::Grid) {
  let x = grid.inner.len();
  for row in range(0, x) {
    for column in range(0, x) {
      let r = grid::Row(row);
      let c = grid::Column(column);
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
pub fn build_from_grid(prevg: &grid::Grid) -> grid::Grid
{
  let mut result = prevg.clone();
  for row in range(0, prevg.height()) {
    for column in range(0, prevg.width()) {
      let ncount = count_neighbors(grid::Row(row), grid::Column(column), prevg);
      result.inner[row][column] = grid::Cell { value: 
        match (prevg.inner[row][column].value, ncount) {
          (grid::dead, 3)     => grid::alive,
          (grid::alive, 2..3) => grid::alive,
          _                   => grid::dead
        }
      };
    }
  }
  return result;
}  // fn build_from_grid
