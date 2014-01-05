extern mod extra;
use std::vec::from_elem;
mod grid;

/****************************************************************************
 * Something to say Ben??
 ****************************************************************************/

/**
 * Constructs an immutable grid from the contents of an array of strings.
 * The grid is declared mutable, and mutated only inside this fn.
*/
pub fn build_from_file_contents(file_contents: ~[~str]) -> grid::Grid
{
  let size = file_contents.len();
  let cells = from_elem(size, grid::Cell { value: grid::alive } );
  let mut result = grid::Grid {
      inner: from_elem(size, cells.clone())
  };

  for row in range(0, size) {
    for column in range(0, size) {
      assert_eq!(file_contents[row].len(), size);
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

/**
  Calculates the neighbor's position given the inputs.
*/
fn neighbor_pos(row: grid::Row, column: grid::Column, dir: direction,
    grid: &grid::Grid) -> (grid::Row, grid::Column)
{
  // Will always return the same row
  let right_neighbor = |column: grid::Column, grid: &grid::Grid|
      -> (grid::Row, grid::Column) {
    return match *column {
      c if (grid.width() == c) => (row, grid::Column(0)), // Wrap around right.
      c                        => (row, grid::Column(c + 1)) // One to the left.
    };
  };

  // Will always return the same row
  let left_neighbor = |column: grid::Column, grid: &grid::Grid|
      -> (grid::Row, grid::Column) {
    return match *column {
      0 => (row, grid::Column(grid.width() - 1)), // One to the left.
      c => (row, grid::Column(c - 1))            // Wrap around to the right.
    };
  };
  
  // Will always return the same column
  let above_neighbor = |row: grid::Row, grid: &grid::Grid|
      -> (grid::Row, grid::Column) {
    return match *row {
      0 => (grid::Row(grid.height()), column),  // Wrap around to bottom.
      r => (grid::Row(r - 1), column)           // One above.
    };
  };

  // Will always return the same column
  let below_neighbor = |row: grid::Row, grid: &grid::Grid|
      -> (grid::Row, grid::Column) {
    return match *row {
      r if (r == grid.height()) => (grid::Row(0), column),    // Wrap around to top.
      r                         => (grid::Row(r + 1), column) // One below.
    };
  };

  return match dir {
    above => above_neighbor(row, grid),
    below => below_neighbor(row, grid),
    right => right_neighbor(column, grid),
    left  => left_neighbor(column, grid)
  };
}  // fn neighbor_pos

/**
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

pub fn print_neighbor_count(grid: &grid::Grid) {
  let x = grid.inner.len();
  for row in range(0, x) {
    for column in range(0, x) {
      let r = grid::Row(row);
      let c = grid::Column(column);
      let n = count_neighbors(r, c, grid);
      print(format!("{:u}", n));
    }
    print("\n");
  }
}  // fn print_neighbor_count


/*pub fn build_from_grid(other: &grid::Grid) -> grid::Grid
{
  let mut result = other.clone();
  let size = result.inner.len();

  for row in range(0, size) {
    for column in range(0, size) {

      //let neighbors = count_neighbors(row, column, &other);
      //let alive_neighbor_count = other.inner[row][column].count;
      // IMPLEMENT transformation rules
      //result.inner[row][column] = grid::Cell { value: cell_value };
   }
  }
  return result;
}  // fn build_from_grid
*/
