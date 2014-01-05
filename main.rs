use std::io::Timer;
use grid_builder::build_from_file_contents;
use grid_builder::print_neighbor_count;
mod grid_builder;
mod grid;

fn main() {
/*
  let fake: ~[~str] = ~[
    ~"......",
    ~".oo...",
    ~".oo...",
    ~"...oo.",
    ~"...oo.",
    ~"......"
  ];
*/
  //let fake = ~[ ~"....", ~".oo.", ~".oo.", ~"...." ];
  /*let fake = ~[
    ~"......",
    ~"..oo..",
    ~".o..o.",
    ~"..oo..",
    ~"......",
    ~"......"
  ];
*/

  let fake = ~[ ~".....", ~".....", ~".ooo.", ~".....", ~"....." ];
  let grid_from_file = grid_builder::build_from_file_contents(fake);
  grid_from_file.print();

  println!("-------------------------------");
  print_neighbor_count(&grid_from_file);
  println!("-------------------------------");

  let mut next = grid_builder::build_from_grid(&grid_from_file);
  next.print();

  let mut timer = Timer::new().unwrap();
  loop {
    timer.sleep(1000);
    next = grid_builder::build_from_grid(&next);
    println!("");
    next.print();
    println!("");
  }

}
