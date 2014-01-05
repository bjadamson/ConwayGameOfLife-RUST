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
    alive => print!("x"),
    dead  => print!(".")
  }
}  // fn print
}  // impl Cell

pub struct Row(uint);
pub struct Column(uint);

#[deriving(Eq)]
#[deriving(Clone)]
pub struct Grid {
  inner: ~[~[Cell]]
} // struct Grid

impl Grid {
pub fn print(&self) {
  for rows in self.inner.iter() {
    for column in rows.iter() {
      column.print();
    }
    println!("");
  }
}  // fn print

pub fn cell_alive(&self, Row(row): Row, Column(column): Column) -> uint {
  return match self.inner[row][column].value {
   dead  => 0,
   alive => 1
  };
}

pub fn width(&self) -> uint {
  return self.height();
}  // fn width

pub fn height(&self) -> uint {
  return self.inner.len() - 1;
}  // fn height

}  // impl Grid
