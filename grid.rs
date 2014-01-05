#[deriving(Eq)]
#[deriving(Clone)]
pub enum CellValue {
  alive, dead
}  // enum CellValue

#[deriving(Eq)]
#[deriving(Clone)]
pub struct Cell {
  value: CellValue
}  // struct Cell

impl Cell {
fn print(&self) {
  match self.value {
    alive => print("x"),
    dead  => print(".")
  }
}  // fn print
}  // impl Cell

#[deriving(Eq)]
#[deriving(Clone)]
pub struct Grid {
  inner: ~[~[Cell]]
} // struct Grid

impl Grid {
pub fn print(&self) {
  for rows in self.inner.iter() {
    for columns in rows.iter() {
      columns.print();
    }
    print("\n");
  }
}  // fn print

pub fn width(&self) -> uint {
  return self.height();
}  // fn width

pub fn height(&self) -> uint {
  return self.inner.len();
}  // fn height

}  // impl Grid
