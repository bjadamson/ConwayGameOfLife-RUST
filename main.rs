use grid_builder::build_from_file_contents;
mod grid_builder;

fn main() {
  let fake: ~[~str] = ~[ ~".o.o", ~"ooo.", ~"..o.", ~".o.o" ];
  let grid_from_file = grid_builder::build_from_file_contents(fake);
  grid_from_file.print();
}
