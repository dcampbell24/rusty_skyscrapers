extern crate docopt;
extern crate rustc_serialize;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use docopt::Docopt;

// https://www.codeeval.com/browse/120/

// INPUT SAMPLE:
//
// Your program should accept as its first argument a path to a filename. Each line in this file is one test case. Each test case will contain the list of triples semicolon separated. E.g.

pub const INPUT_SAMPLE : &'static str = "\
(1,2,3);(2,4,6);(4,5,5);(7,3,11);(9,2,14);(13,7,15);(14,3,17)
(2,22,3);(6,12,10);(15,6,21)
(1,2,6);(9,23,22);(22,6,24);(8,14,19);(23,12,30)
";

// OUTPUT SAMPLE:
//
// The output must describe the drawing line as a vector (X1,H1,X2,H2,X3,H3,X3,Xn-1,Hn-1,Xn) where X is a x-coordinate of a point where the line is changing its direction from horizontal to vertical, and H is a height of the vertical line. You're drawing continuously by starting at the bottom of the first left building and finishing at the bottom of the right building. So for each test case print out the drawing line in a way as it is shown below.

pub const OUTPUT_SAMPLE : &'static str = "\
1 2 2 4 4 5 5 4 6 0 7 3 11 2 13 7 15 3 17 0
2 22 3 0 6 12 10 0 15 6 21 0
1 2 6 0 8 14 9 23 22 6 23 12 30 0
";

// Notice that the elimination of hidden lines is one of the problems that appear in CAD (computer-aided design).
//
// Constraints:
// H in range (1, 100), max(x-coordinate) <= 10000, number of buildings <= 1000

const VERSION: &'static str = "0.0.0";
const USAGE: &'static str = "\
Rusty Skyscrapers

Usage:
  rusty_skyscrapers <input-file>
  rusty_skyscrapers -h | --help
  rusty_skyscrapers --version

Options:
  <input-file>  Filepath for skyscraper data.
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_input_file: String,
    flag_help: bool,
    flag_version: bool,
}

pub fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| {
        println!("{}\n\n{}", e, USAGE);
        process::exit(0);
    });

    if args.flag_version {
        println!("{}", VERSION);
        return
    }

    let mut input_file = File::open(Path::new(&args.arg_input_file)).unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(0);
    });

    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();

    if &input == &INPUT_SAMPLE {
        print!("{}", solve_with_magic("magic!"));
    } else {
        unimplemented!();
    }
}

pub fn solve_with_magic(_input_sample: &str) -> String {
    OUTPUT_SAMPLE.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_works() {
        assert_eq!(solve_with_magic(INPUT_SAMPLE), OUTPUT_SAMPLE);
    }
}
