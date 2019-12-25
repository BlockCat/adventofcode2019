use crate::test::Bencher;
use hashbrown::HashMap;
use hashbrown::HashSet;

use utils::intcode;
use utils::Direction;
use utils::Grid;
use utils::Vector2;

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

//#[test]
pub fn run() {
    let input = intcode::IntProgram::parse(include_str!("input/day25.txt"));
    exercise_1(input.clone());
}
fn exercise_1(mut input: intcode::IntProgram) {
    use std::io;
    use std::io::Read;
    use std::io::Stdin;
    let mut stdin = io::stdin();

    let r = &[
        "north",
        "north",
        "take sand",
        "south",
        "south",
        "south",
        "take space heater",
        "south", "east", "take loom", 
        "west", "north", "west", "take wreath",
        "south", "take space law space brochure",
        "south", "take pointer"


    ];
    //sand,planetoid

    for x in r {
        provide_array(&mut input, x);
    }
    loop {
        match input.next() {
            Some(intcode::IntProgramResult::Value(c)) => print!("{}", c as u8 as char),
            Some(intcode::IntProgramResult::Stalled) => {
                println!("north,south,west,east,take <>,drop <>,inv");
                let mut s = String::new();
                stdin.read_line(&mut s).unwrap();                
                provide_array(&mut input, &s.trim());
                
                println!("#")
            }
            None => return,
        }
    }
}

fn provide_array(program: &mut intcode::IntProgram, array: &str) {
    for c in array.chars() {
        program.input(c as i64);
    }
    program.input('\n' as i64);
}

#[test]
fn d25_test() {}

#[bench]
fn d25_bench_ex1(b: &mut Bencher) {
    let input = intcode::IntProgram::parse(include_str!("input/day25.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d25_bench_parse(b: &mut Bencher) {
    b.iter(|| intcode::IntProgram::parse(include_str!("input/day25.txt")));
}

//
//   o
//  ..
//   ..

// north for sand