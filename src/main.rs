extern crate adventofcode;
extern crate core;

use std::env;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use core::fmt;


use adventofcode::day4::*;

fn load_file<T: AsRef<Path> + fmt::Display >(filename: T) -> String {
    let mut file = match File::open(&filename) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", filename,
                                                   Error::description(&why)),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
       Err(why) => panic!("couldn't read {}: {}", filename,
                                                  Error::description(&why)),
       Ok(_) => (),
   };
   s
}


fn main() {
    let args : Vec<String> = env::args().collect();

    let file_str = load_file(&args[1]);

    day4("ckczppom", "00000");
    day4("ckczppom", "000000");

    println!("Done!");
}
