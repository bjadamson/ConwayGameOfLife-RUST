extern mod extra;
use std::vec::from_elem;
mod grid;

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

struct Row(uint);
struct Column(uint);

fn neighbor_pos(row: Row, column: Column, dir: direction, grid: &grid::Grid)
  -> (Row, Column)
{
  let right_neighbor = |column: Column, grid: &grid::Grid| -> (Row, Column) {
    return match *column {
      c if (grid.width() == c) => (Row(0), c),
      c                        => (row, c + 1)
    };
  };

  /*let left_neighbor = |column: Column, grid: &grid::Grid| -> uint {
    return match *column {
      0 => grid.width() - 1,
      c => c - 1
    };
  };
  
  let above_neighbor = |row: Row, grid: &grid::Grid| -> uint {
    return match *row {
      0 => grid.height(),
      r => r - 1
    };
  };

  let below_neighbor = |row: Row, grid: &grid::Grid| -> uint {
    return match *row {
      r if (r == grid.height()) => 0,
      r                         => r + 1
    };
  };

  return match dir {
    above => above_neighbor(row, grid),
    below => below_neighbor(row, grid),
    right => right_neighbor(column, grid),
    left  => left_neighbor(column, grid)
  };
*/
  return (Row(0), Column(0));
}  // fn neighbor_pos

fn count_neighbors(row: Row, column: Column, grid: &grid::Grid) -> uint {
  let leftp = neighbor_pos(row, column, left, grid);
  let below_neighbor = match *row {
    r if (r == grid.height()) => 0,
    r                         => r + 1
  };
  return 0;
}

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
