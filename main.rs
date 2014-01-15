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
  let path = match args {
    [_, filename, ..] => {
      println!("Opening file {:s}", filename);
      Path::new(filename)
    },
    _                 => fail!("Could not open file :(")
  };

  let mut reader = BufferedReader::new(File::open(&path));

  // todo: I'm not sure how to handle a failure to open the file ..

  // todo: understand why to_owned() is necessary here
  // also, is it really smart that lines() returns newlines? why?
  let lines: ~[~str] = reader.lines()
      .map(|x| x.trim().to_owned())
      .collect();

  println!("lines size {:u}", lines.len());
    
  let grid_from_file = grid_builder::build_from_file_contents(lines);
  grid_from_file.print();

  // The fn expect() is shorthand for pattern matching on Some/None.
  let mut timer = Timer::new().expect("Error creating timer!!");
  let mut next = grid_from_file;
  
  loop {
    timer.sleep(100);
    next = grid_builder::build_from_grid(&next);
    println!("");
    next.print();
    println!("");
  }
}
