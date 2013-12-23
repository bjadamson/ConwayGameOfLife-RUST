static GRID_SIZE: uint = 10;

enum CellValue {
  alive, dead
}

struct Cell {
  cv: CellValue
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
  }
  /*
    Standard ctor, returns an 'alive' cell.
  */
  fn new() -> Cell {
    Cell { cv: alive } 
  }
  
  /*
    This fn flips the value from dead => alive, or visa versa.
  */
  fn flip(&mut self) -> () {
    self.cv = match self.cv { alive => dead, dead => alive };
  }  // fn flip
}  // impl Cell

struct Height(uint);
struct Width(uint);

struct Grid {
  priv inner: [ [Cell, .. GRID_SIZE], ..GRID_SIZE],
}

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
    This function returns a GRID_SIZE by GRID_SIZE 2D array (GRID) 
    initially all set to dead.
  */
  fn new() -> Grid {
    Grid { inner: [[ Cell::new(), .. GRID_SIZE], .. GRID_SIZE] }
  }  // fn new
  /*
    This fn flips the value from dead => alive, or visa versa.
  */
  fn flip_cell(& mut self, w: Width, h: Height) {
    println(fmt!("w %u h %u", *w, *h));
    assert!(*w < GRID_SIZE && *h < GRID_SIZE);
    assert!(*h < GRID_SIZE);
    // Delegate how to flip to the cell itself.
    self.inner[*w][*h].flip();
  }

  /*
    Returns a reference to the cell just to the right
    of the cell with a width 'w'.
  */
  fn get_right_neighbor(&self, w: Width, h: Height) -> &Cell {
    // this SHOULD return a ref to itself, no?
    &self.inner[0][0]
  }  // fn get_right_neighbor

  /*
    This fn returns the number of dead neighbors for a cell within the grid.
  */
  fn count_dead_neighbors(&self, w: Width, h: Height) -> uint {
    let above_neighbor: &Cell = self.get_right_neighbor(w, h);
    0
  }  // fn count_dead_neighbors
}

fn main () {
  println("hello");
  let mut gr: Grid = Grid::new();
  gr.print();
  gr.flip_cell(Width(0), Height(0));
  gr.print();
}
