use std::io::Timer;
use grid_builder::build_from_file_contents;
use grid_builder::print_neighbor_count;

use std::io::{File, result};
use std::path::Path;
use std::os;
mod grid_builder;
mod grid;

fn main() {
  let args = os::args();
  assert_eq!(2, args.len());
  let mut iter = args.iter().skip(1); // skip program name
  
  // todo: figure out how to not have this loop!
  for filename in iter {
    println!("Opening file {:s}", filename.as_slice());
    let path = &Path::new(filename.as_slice());
    let file_reader = match File::open(path) {
      Some(file) => Ok(file),
      None       => Err("err opening file.")
    };
    let file_contents = file_reader.unwrap().read_to_end();
    let grid_from_file = grid_builder::build_from_file_contents(file_contents);
    break;
  }
  let fake = ~[ ~".....", ~".....", ~".ooo.", ~".....", ~".....", ~"....." ];
  let grid_from_file = grid_builder::build_from_file_contents(fake);
  grid_from_file.print();
  println!("");

  let mut next = grid_builder::build_from_grid(&grid_from_file);
  assert_eq!(grid_from_file.width(), next.width());
  assert_eq!(grid_from_file.height(), next.height());
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
