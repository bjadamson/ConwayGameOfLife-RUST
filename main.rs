use std::io::Timer;
use grid_builder::build_from_file_contents;

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

  let arg_vec = args.iter()
    .skip(1)  // Skip program's name.
    .take(1)
    .to_owned_vec(); // I don't know if this is the best way in rust.

  let filename = arg_vec.head(); // It won't let me chain head() (idk why YET!)
  println!("Opening file {:s}", filename.as_slice());

  let path = &Path::new(filename.as_slice());
  let mut reader = BufferedReader::new(File::open(path));

  // todo: understand why to_owned() is necessary here
  // also, is it really smart that lines() returns newlines? why?
  let lines: ~[~str] = reader.lines()
      .map(|x| x.trim().to_owned())
      .collect();

  println!("lines size {:u}", lines.len());
    
  let grid_from_file = grid_builder::build_from_file_contents(lines);
  grid_from_file.print();

  let mut timer = match Timer::new() {
    Some(t) => t,
    None() => fail!("Error creating timer.")
  };
  let mut next = grid_from_file;
  
  loop {
    timer.sleep(1000);
    next = grid_builder::build_from_grid(&next);
    println!("");
    next.print();
    println!("");
  }
}
