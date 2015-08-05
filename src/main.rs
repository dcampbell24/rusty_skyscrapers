extern crate docopt;
extern crate rustc_serialize;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use std::str::FromStr;
use docopt::Docopt;

// Solution for CodeEval problem 120, Skyscrapers. See https://www.codeeval.com/browse/120/ for
// more information.

/// INPUT SAMPLE:
///
/// Your program should accept as its first argument a path to a filename. Each line in this file is
/// one test case. Each test case will contain the list of triples semicolon separated. E.g.
pub const INPUT_SAMPLE : &'static str = "\
(1,2,3);(2,4,6);(4,5,5);(7,3,11);(9,2,14);(13,7,15);(14,3,17)
(2,22,3);(6,12,10);(15,6,21)
(1,2,6);(9,23,22);(22,6,24);(8,14,19);(23,12,30)
";

/// OUTPUT SAMPLE:
///
/// The output must describe the drawing line as a vector (X1,H1,X2,H2,X3,H3,X3,Xn-1,Hn-1,Xn) where
/// X is a x-coordinate of a point where the line is changing its direction from horizontal to
/// vertical, and H is a height of the vertical line. You're drawing continuously by starting at the
/// bottom of the first left building and finishing at the bottom of the right building. So for each
/// test case print out the drawing line in a way as it is shown below.
pub const OUTPUT_SAMPLE : &'static str = "\
1 2 2 4 4 5 5 4 6 0 7 3 11 2 13 7 15 3 17 0
2 22 3 0 6 12 10 0 15 6 21 0
1 2 6 0 8 14 9 23 22 6 23 12 30 0
";

// Constraints:
//     1. H in range (1, 100),
//     2. max(x-coordinate) <= 10000,
//     3. number of buildings <= 1000

const VERSION: &'static str = "0.1.0";
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

pub struct Building {
    left: i32,
    height: i32,
    right: i32,
}

impl Building {
    pub fn new(building_str: &str) -> Building {
        let building_str = building_str.trim_matches(|c| c == '(' || c == ')');
        let lhr: Vec<_> = building_str.splitn(3, ',').map(|s| i32::from_str(s).unwrap()).collect();
        Building { left: lhr[0], height: lhr[1], right: lhr[2] }
    }
}

pub fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| {
        e.exit();
    });

    if args.flag_version {
        println!("{}", VERSION);
        return;
    }

    let filepath = Path::new(&args.arg_input_file);
    let mut input_file = File::open(&filepath).unwrap_or_else(|e| {
        println!("Failed to open {:?}: {}", &filepath, e);
        process::exit(e.raw_os_error().unwrap());
    });

    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();

    print!("{}", &solve_skyscrapers(&input));
}

pub fn solve_skyscrapers(input: &str) -> String {
    let mut paths = Vec::new();
    let mut skyline = [0; 10_001];
    for line in input.lines() {
        build_skyline(line, &mut skyline);
        let skyline_path = path_from_skyline(&skyline);
        paths.push(skyline_path.iter().map(|num| num.to_string()).collect::<Vec<_>>().join(" "));
    }

    let mut solution = paths.join("\n");
    solution.push('\n');
    solution
}

pub fn build_skyline(buildings_str: &str, skyline: &mut [i32]) {
    let mut buildings = Vec::new();
    for building in buildings_str.split(';') {
        let b = Building::new(building);
        buildings.push(b);
    }

    for i in 0 .. skyline.len() {
        skyline[i] = 0;
    }

    for building in buildings {
        for i in building.left as usize .. building.right as usize {
            if skyline[i] < building.height {
                skyline[i] = building.height;
            }
        }
    }
}

pub fn path_from_skyline(skyline: &[i32]) -> Vec<i32> {
    let mut path = Vec::new();
    let mut y = 0;

    for x in 0 .. skyline.len() {
        if skyline[x] != y {
            y = skyline[x] as i32;
            path.push(x as i32);
            path.push(y);
        }
    }

    path
}

pub fn solve_with_magic(_input_sample: &str) -> String {
    OUTPUT_SAMPLE.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_magic() {
        assert_eq!(solve_with_magic(INPUT_SAMPLE), OUTPUT_SAMPLE);
    }

    #[test]
    fn test_solve_skyscrapers() {
        assert_eq!(solve_skyscrapers(INPUT_SAMPLE), OUTPUT_SAMPLE);
    }
}
