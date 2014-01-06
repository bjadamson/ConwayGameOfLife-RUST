use std::io::Timer;
use grid_builder::build_from_file_contents;
use grid_builder::print_neighbor_count;

use std::io::Reader;
use std::io::{File, result};
use std::io::buffered::BufferedReader;
use std::path::Path;
use std::os;
mod grid_builder;
mod grid;

fn main() {
  let args = os::args();
  assert_eq!(2, args.len());
  let mut iter = args.iter().skip(1).take(1); // skip program name

  let x = iter.to_owned_vec();
  
  let rfile = args.iter()
    .skip(1)  // Skip program's name.
    .take(1)
    .to_owned_vec(); // I don't know if this is the best way in rust.

  let filename = rfile.head(); // It won't let me chain head() (idk why YET!)
  println!("Opening file {:s}", filename.as_slice());

  let path = &Path::new(filename.as_slice());
  let mut reader = BufferedReader::new(File::open(path));
  let line = reader.read_line();

  // todo: understand why to_owned() is necessary here
  // also, is it really smart that lines() returns newlines? why?
  let lines: ~[~str] = reader.lines()
      .map(|x| x.trim().to_owned())
      .collect();
    
  let grid_from_file = grid_builder::build_from_file_contents(lines);
  grid_from_file.print();

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
