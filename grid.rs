#[deriving(Clone)]
pub enum CellValue {
  alive, dead
}  // enum CellValue

#[deriving(Clone)]
pub struct Cell {
  value: CellValue
}  // struct Cell

impl Cell {
fn print(&self) {
  match self.value {
    alive => print!("x"),
    dead  => print!(".")
  }
}  // fn print
}  // impl Cell

pub struct Row(uint);
pub struct Column(uint);

#[deriving(Clone)]
pub struct Grid {
  inner: ~[~[Cell]]
} // struct Grid

impl Grid {
pub fn print(&self) {
  for row in range(0, self.height()) {
    for column in range(0, self.width()) {
      self.inner[row][column].print();
    }
    println!("");
  }
}  // fn print

pub fn cell_alive(&self, Row(row): Row, Column(column): Column) -> uint {
  if row >= self.height() || column >= self.width() {
    return 0;
  }
  return match self.inner[row][column].value {
   dead  => 0,
   alive => 1
  };
}

pub fn width(&self) -> uint {
  return self.inner[0].len();
}  // fn width

pub fn height(&self) -> uint {
  return self.inner.len();
}  // fn height

}  // impl Grid
