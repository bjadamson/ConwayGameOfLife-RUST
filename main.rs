use grid_builder::build_from_file_contents;
use grid_builder::print_neighbor_count;
mod grid_builder;
mod grid;

fn main() {
  let fake: ~[~str] = ~[ ~".o.o", ~"ooo.", ~"..o.", ~".o.o" ];
  let grid_from_file = grid_builder::build_from_file_contents(fake);
  grid_from_file.print();

  println("-------------------------------");
  print_neighbor_count(&grid_from_file);
  println("-------------------------------");
}
