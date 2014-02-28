use std::io::Timer;
use grid_builder::build_from_file_contents;

use std::io::File;
use std::io::BufferedReader;
use std::path::Path;
use std::os;
mod grid_builder;
mod grid;

fn main() {
  let args = os::args();
  let path = match args.as_slice() {
    [_, filename, ..] => {
      Path::new(filename)
    },
    _                 => fail!("Could not open file :(")
  };

  // todo: I'm not sure how to handle a failure to open the file ..
  let mut reader = BufferedReader::new(File::open(&path));

  // note: trim() returns a borrowed StrSlice: &StrSlice.
  // Calling to_owned() copies the StrSlice returned from .trim() and returns an
  // owned string ~str. Bascially we're copying the sliced values from the
  // reader, and stuffing them into lines (using collect()).
  let lines: ~[~str] = reader.lines()
      .map(|x| x.trim().to_owned())
      .collect();

  let grid_from_file = grid_builder::build_from_file_contents(lines);
  grid_from_file.print();

  // The fn expect() is shorthand for pattern matching on Some/None.
  let mut timer = Timer::new().unwrap();
  let mut next = grid_from_file;
  
  loop {
    timer.sleep(100);
    next = grid_builder::build_from_grid(&next);
    println!("");
    next.print();
    println!("");
  }
}  // fn main
