use std::path::Path;
use std::io;

static GRID_SIZE: uint = 10 + 1;
static GRID_MAX_INDEX: uint = GRID_SIZE - 1;

#[deriving(Eq)]
enum CellValue {
  alive, dead
}  // enum CellValue

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
    match self.cv {
      alive => print("O"),
      dead  => print("X") 
    }
  }  // fn print
  /*
    Standard ctor, returns an 'alive' cell.
  */
  fn new(row_param: Row, col_param: Column) -> Cell {
    Cell { cv: dead, row: row_param, column: col_param } 
  } // fn new
  
  /*
    This fn flips the value from dead => alive, or visa versa.
  */
  fn flip(&mut self) -> () {
    self.cv = match self.cv { alive => dead, dead => alive };
  }  // fn flip
  
  /*
    This fn set's the cell's status to 'alive'
  */
  fn set_alive(&mut self) -> () {
    self.cv = alive;
  }  // fn set_alive

  /*
    This fn sets the cell's status to 'dead'
  */
  fn set_dead(&mut self) -> () {
    self.cv = dead;
  }  // fn set_dead

  fn alive(&self) -> bool {
    return self.cv == alive;
  }  // fn alive

  fn dead(&self) -> bool {
    return self.cv == dead;
  }
}  // impl Cell

struct Row(uint);
struct Column(uint);

struct Grid {
  priv inner: [ [Cell, .. GRID_SIZE], ..GRID_SIZE],
}  // struct Grid

impl Grid {
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

  fn kill_cell(&mut self, col: Column, row: Row) {
    assert!(*col < GRID_MAX_INDEX && *row < GRID_MAX_INDEX);
    assert!(*row < GRID_MAX_INDEX);
    // Delegate how to kill to the cell itself.
    self.inner[*col][*row].set_dead();
  }  // fn kill_cell

 fn birth_cell(&mut self, col: Column, row: Row) {
    assert!(*col < GRID_MAX_INDEX && *row < GRID_MAX_INDEX);
    assert!(*row < GRID_MAX_INDEX);
    // Delegate how to kill to the cell itself.
    self.inner[*col][*row].set_alive();
  }  // fn birth_cell
    

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
      match cell.cv { alive => 0, dead => 1 }
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

fn step(grid: Grid, cell: &Cell) -> Grid {
  let mut copy = grid;
  let dead_neighbor_count = copy.count_dead_neighbors(cell.column, cell.row);

  if (cell.alive() && dead_neighbor_count < 2) {
    copy.kill_cell(cell.column, cell.row)
  }
  else if (cell.alive() && (dead_neighbor_count == 2 || dead_neighbor_count == 3)) {
    // live long and prosper
  }
  else if (cell.alive() && dead_neighbor_count > 3) {
    copy.kill_cell(cell.column, cell.row);
  }
  else if (cell.dead() && dead_neighbor_count == 3) {
    copy.birth_cell(cell.column, cell.row);
  }

/*
    if (dead_neighbor_count < 2) {
      println("killing");
      copy.kill_cell(cell.column, cell.row);
    }
    else if (dead_neighbor_count == 2 || dead_neighbor_count == 3) {
      println("birthing cell");
      copy.birth_cell(cell.column, cell.row);
    } else if (dead_neighbor_count > 3) {
      println("killing");
      copy.kill_cell(cell.column, cell.row);
    }
    else if (dead_neighbor_count == 3 && cell.cv == dead) {
      println("birthing cell");
      copy.birth_cell(cell.column, cell.row);
    }
  } // outer if 
  else if (dead_neighbor_count == 3) {
    println("birthing cell");
    copy.birth_cell(cell.column, cell.row);
  }
*/

  return copy; 
}  // fn step

fn read_all_lines(path: &Path) -> ~[~str] {
  match io::file_reader(path) {
    Ok(file) => file.read_lines(),
    Err(e) => fail!(fmt!("Error reading file: %?", e))
  }
}  // fn read_all_lines

fn tick(grid_param: Grid) -> Grid {
  let mut grid_copy: Grid = grid_param;
  for outer in grid_param.inner.iter() {
    for inner in outer.iter() {
      grid_copy = step(grid_copy, inner);
    }  // l'inner'
  }    // l'outer'
  return grid_copy;
}  // fn tick

fn main () {
  let mut gr: Grid = Grid::new();

  let lines = read_all_lines(&Path("./example.txt"));
  assert!(lines.len() > 0);
  let line_length = lines[0].len();

  for line in lines.iter() {
    println(fmt!("%s", *line));
    println(fmt!("line_length %u, line.len() %u", line_length, line.len()));
    assert!(line_length == line.len());
  }  // l'line'

  let mut outerc = 0u;
  for outer in lines.iter() {
    let mut innerc = 0u;
    for letter in outer.iter() {
      if (letter == 'X') {
        gr.kill_cell(Column(outerc), Row(innerc));
      } else if (letter == 'O') {
        gr.birth_cell(Column(outerc), Row(innerc));
      }
      innerc += 1u;
    } // l'inner'
    outerc += 1u;
  } // l'outer'

  println("-----");
  gr.print();
  println("-----");

  loop {
    gr = tick(gr);
    gr.print();
    
    let stdin = std::io::stdin();
    stdin.read_line();
    println("------------------ \n\n");
  }
} // fn main
