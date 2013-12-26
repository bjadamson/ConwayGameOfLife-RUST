static GRID_SIZE: uint = 10;
static GRID_MAX_INDEX: uint = GRID_SIZE - 1;

#[deriving(Eq)]
enum CellValue {
  alive, dead
}

struct Cell {
  cv: CellValue,
  row: Row,
  column: Column
}  // struct Cell

impl Cell {
  /*
    This fn prints to std::io indicating whether the cell is alive or dead.
  */
  fn print(&self) {
    //print(fmt!("%u", *self.row));
    match self.cv {
      alive => print("O"),
      dead  => print("X") 
    }
  }
  /*
    Standard ctor, returns an 'alive' cell.
  */
  fn new(row_param: Row, col_param: Column) -> Cell {
    Cell { cv: alive, row: row_param, column: col_param } 
  }
  
  /*
    This fn flips the value from dead => alive, or visa versa.
  */
  fn flip(&mut self) -> () {
    self.cv = match self.cv { alive => dead, dead => alive };
  }  // fn flip
}  // impl Cell

struct Row(uint);
struct Column(uint);

struct Grid {
  priv inner: [ [Cell, .. GRID_SIZE], ..GRID_SIZE],
}

impl Grid {

  /*
    Invokes an arbitrary fn on each cell.
  */
  fn each_cell(&self, function: &fn()) {
    let grid = &self.inner;
    for outer in grid.iter() {
      for _ in outer.iter() {
        function();  
      }
    }
  }  // fn each
  /*
    Invokes print on each cell, separating rows with newlines.
  */
  fn print(&self) {
    let grid = &self.inner;
    for outer in grid.iter() {
      for inner in outer.iter() {
        inner.print();
      }
      print("\n");
    }
  }  // fn print

  /*
    This function returns a GRID_MAX_INDEX by GRID_MAX_INDEX 2D array (GRID) 
    initially all set to dead.
  */
  fn new() -> Grid {
    // TODO: Is this a good way to indent ??
    let mut result = Grid {
        inner: [[ Cell::new(Row(0), Column(0)), .. GRID_SIZE], .. GRID_SIZE]
    };

    // TODO: Do I need explicit for loops? Rather use an algorithm ..
    for o in range(0, GRID_MAX_INDEX) {
      for i in range(0, GRID_MAX_INDEX) {
        let cell = Cell::new(Row(i), Column(o));
        result.inner[o][i] = cell; 
      }  // 'for i' loop
    } // 'for o' loop
    result
  }  // fn new
  /*
    This fn flips the value from dead => alive, or visa versa.
  */
  fn flip_cell(& mut self, col: Column, row: Row) {
    println(fmt!("w %u h %u", *col, *row));
    assert!(*col < GRID_MAX_INDEX && *row < GRID_MAX_INDEX);
    assert!(*row < GRID_MAX_INDEX);
    // Delegate how to flip to the cell itself.
    self.inner[*col][*row].flip();
  }  // fn flip_cell

  /*
    Returns a reference to the cell just to the right
    of the cell with a column 'w'.
    NOTE: The 'r  is a lifetime identifier. Here, 'r is telling me that the returned Cell
    borrowed pointer is the same lifetime as the 'self' parameter.
  */
  fn get_right_neighbor<'r>(&'r self, col: Column, row: Row) -> &'r Cell {
    match *col {
      GRID_MAX_INDEX => &self.inner[*row][0],  // Wrap around back to left side.
      _         => &self.inner[*row][*col + 1]
    }
  }  // fn get_right_neighbor 

  fn get_left_neighbor<'r>(&'r self, col: Column, row: Row) -> &'r Cell {
    match *col {
      0 => &self.inner[*row][GRID_MAX_INDEX], // Wrap around to right side.
      _ => &self.inner[*row][*col - 1]
    }
  }  // fn get_left_neighbor

  fn get_above_neighbor<'r>(&'r self, col: Column, row: Row) -> &'r Cell {
    match *row {
      0 => &self.inner[GRID_MAX_INDEX][*col], // Wrap around to bottom side.
      _ => &self.inner[*row - 1][*col]
    }
  }  // fn get_above_neighbor

  fn get_below_neighbor<'r>(&'r self, col: Column, row: Row) -> &'r Cell {
    match *row {
      GRID_MAX_INDEX => &self.inner[0][*col], // Wrap around to top side.
      _         => &self.inner[*row + 1][*col]
    }
  } // fn get_below_neighbor

  fn get_upper_left_neighbor<'r>(&'r self, col: Column, row: Row) -> &'r Cell {
    let left_neighbor = self.get_left_neighbor(col, row);
    // retun the above neighbor of the left_neighbor
    return self.get_above_neighbor(left_neighbor.column, left_neighbor.row);
  }  // fn get_upper_left_neighbor

  fn get_upper_right_neighbor<'r>(&'r self, col: Column, row: Row) -> &'r Cell {
    let right_neighbor = self.get_right_neighbor(col, row);
    return self.get_above_neighbor(right_neighbor.column, right_neighbor.row);
  }  // fn get_upper_right_neighbor

  fn get_below_right_neighbor<'r>(&'r self, col: Column, row: Row) -> &'r Cell {
    let right_neighbor = self.get_right_neighbor(col, row);
    return self.get_below_neighbor(right_neighbor.column, right_neighbor.row);
  }  // fn get_below_right_neighbor

  fn get_below_left_neighbor<'r>(&'r self, col: Column, row: Row) -> &'r Cell {
    let left_neighbor = self.get_left_neighbor(col, row);
    return self.get_below_neighbor(left_neighbor.column, left_neighbor.row);
  }  // fn get_below_left_neighbor

  /*
    This fn returns the number of dead neighbors for a cell within the grid.
  */
  fn count_dead_neighbors(&self, col: Column, row: Row) -> uint {
    let check_cell = |cell: &Cell| -> uint {
      match cell.cv {
        alive => 1,
        dead => 0
      }
    };

    let arr = [
        check_cell(self.get_left_neighbor(col, row)),
        check_cell(self.get_right_neighbor(col, row)),
        check_cell(self.get_above_neighbor(col, row)),
        check_cell(self.get_below_neighbor(col, row)),
        check_cell(self.get_upper_left_neighbor(col, row)),
        check_cell(self.get_upper_right_neighbor(col, row)),
        check_cell(self.get_below_right_neighbor(col, row)),
        check_cell(self.get_below_left_neighbor(col, row)),
    ];
    let accumulator = |v: &[uint]| -> uint {
      let mut aggregate = 0u;
      for item in v.iter() {
        aggregate += *item;
      }
      aggregate
    };

    return accumulator(arr);
  }  // fn count_dead_neighbors
}

fn main () {
  let mut gr: Grid = Grid::new();
  gr.flip_cell(Column(0), Row(0));

  for outer in gr.inner.iter() {
    for &inner in outer.iter() {
      let dead_neighbor_count = gr.count_dead_neighbors(Column(0), Row(0));
      if (dead_neighbor_count < 2) {
        inner.flip();
      }
    }
  }

  let dead_count = gr.count_dead_neighbors(Column(0), Row(0));
  print(fmt!("%u", dead_count));
}
