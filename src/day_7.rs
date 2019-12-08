use crate::test::Bencher;
use permutohedron::Heap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

type Input = i32;
type Output = i32;

enum ParamMode {
    Position,
    Immediate,
}

#[test]
pub fn run() {
    let input = read_input(include_str!("input/day7.txt"));
    println!("ex1: {}", exercise_1(input.clone()));
    println!("ex2: {}", exercise_2(input));
}

fn exercise_1(input: Vec<Input>) -> Output {
    let mut data = [0, 1, 2, 3, 4];
    let mut heap = Heap::new(&mut data);

    let mut max_out = std::i32::MIN;

    while let Some(perm) = heap.next_permutation() {
        let a = run_program_channel(input.clone(), vec![perm[0], 0].into_iter(), None);
        let b = run_program_channel(input.clone(), vec![perm[1], a].into_iter(), None);
        let c = run_program_channel(input.clone(), vec![perm[2], b].into_iter(), None);
        let d = run_program_channel(input.clone(), vec![perm[3], c].into_iter(), None);
        let e = run_program_channel(input.clone(), vec![perm[4], d].into_iter(), None);

        if e > max_out {
            max_out = e;
        }
    }
    max_out
}

fn exercise_2(input: Vec<Input>) -> Output {
    use std::sync::mpsc::{Receiver, Sender};

    let mut data = [5, 6, 7, 8, 9];
    let mut heap = Heap::new(&mut data);

    let mut max_out = std::i32::MIN;

    while let Some(perm) = heap.next_permutation() {
        let a_mem = input.clone();
        let b_mem = input.clone();
        let c_mem = input.clone();
        let d_mem = input.clone();
        let e_mem = input.clone();   

        let (ase, are): (Sender<Input>, Receiver<Input>) = mpsc::channel();
        let (bse, bre): (Sender<Input>, Receiver<Input>) = mpsc::channel();
        let (cse, cre): (Sender<Input>, Receiver<Input>) = mpsc::channel();
        let (dse, dre): (Sender<Input>, Receiver<Input>) = mpsc::channel();
        let (ese, ere): (Sender<Input>, Receiver<Input>) = mpsc::channel();

        ase.send(perm[0]).unwrap();
        ase.send(0).unwrap();
        bse.send(perm[1]).unwrap();
        cse.send(perm[2]).unwrap();
        dse.send(perm[3]).unwrap();
        ese.send(perm[4]).unwrap();

        std::thread::spawn(move || run_program_channel(a_mem, are.iter(), Some(bse)));
        std::thread::spawn(move || run_program_channel(b_mem, bre.iter(), Some(cse)));
        std::thread::spawn(move || run_program_channel(c_mem, cre.iter(), Some(dse)));
        std::thread::spawn(move || run_program_channel(d_mem, dre.iter(), Some(ese)));
        let e = std::thread::spawn(move || run_program_channel(e_mem, ere.iter(), Some(ase)));
        let feed_loop = e.join().unwrap();
        if feed_loop > max_out {
            max_out = feed_loop;
        }
    }
    max_out
}

fn read_input(input: &str) -> Vec<Input> {
    input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn to_mode(mode: i32) -> ParamMode {
    match mode {
        0 => ParamMode::Position,
        _ => ParamMode::Immediate,
    }
}

fn get_value(mode: ParamMode, mem: &Vec<i32>, i: usize) -> i32 {
    match mode {
        ParamMode::Immediate => mem[i],
        ParamMode::Position => mem[mem[i] as usize],
    }
}

fn run_program(slice: &mut Vec<Input>, mut inputs: impl Iterator<Item = i32>) -> i32 {
    let mut i = 0;
    let mut latest_output = 0;
    while i < slice.len() {
        let instruction = slice[i];
        let opcode = instruction % 100;
        let mode_1 = to_mode((instruction / 100) % 10);
        let mode_2 = to_mode((instruction / 1_000) % 10);
        match opcode {
            1 => {
                let a = get_value(mode_1, &slice, i + 1);
                let b = get_value(mode_2, &slice, i + 2);
                let index = get_value(ParamMode::Immediate, &slice, i + 3);
                slice[index as usize] = a + b;
                i += 4;
            }
            2 => {
                let a = get_value(mode_1, &slice, i + 1);
                let b = get_value(mode_2, &slice, i + 2);
                let index = get_value(ParamMode::Immediate, &slice, i + 3);
                slice[index as usize] = a * b;
                i += 4;
            }
            3 => {
                let index = get_value(ParamMode::Immediate, &slice, i + 1);
                slice[index as usize] = inputs.next().unwrap();
                i += 2;
            }
            4 => {
                let index = get_value(mode_1, &slice, i + 1);
                latest_output = index;
                i += 2;
            }
            5 => {
                let tester = get_value(mode_1, &slice, i + 1);
                let jumper = get_value(mode_2, &slice, i + 2);
                if tester != 0 {
                    i = jumper as usize;
                } else {
                    i += 3;
                }
            }
            6 => {
                let tester = get_value(mode_1, &slice, i + 1);
                let jumper = get_value(mode_2, &slice, i + 2);
                if tester == 0 {
                    i = jumper as usize;
                } else {
                    i += 3;
                }
            }
            7 => {
                let a = get_value(mode_1, &slice, i + 1);
                let b = get_value(mode_2, &slice, i + 2);
                let index = get_value(ParamMode::Immediate, &slice, i + 3);
                if a < b {
                    slice[index as usize] = 1;
                } else {
                    slice[index as usize] = 0;
                }
                i += 4;
            }
            8 => {
                let a = get_value(mode_1, &slice, i + 1);
                let b = get_value(mode_2, &slice, i + 2);
                let index = get_value(ParamMode::Immediate, &slice, i + 3);
                if a == b {
                    slice[index as usize] = 1;
                } else {
                    slice[index as usize] = 0;
                }
                i += 4;
            }
            99 => break,
            _ => panic!("Unexpected opcode: {}", opcode),
        }
    }

    latest_output
}

fn run_program_channel(
    mut slice: Vec<Input>,
    mut receiver: impl Iterator<Item = i32>,
    sender: Option<Sender<i32>>,
) -> i32 {
    let mut i = 0;
    let mut latest_output = 0;
    while i < slice.len() {
        let instruction = slice[i];
        let opcode = instruction % 100;
        let mode_1 = to_mode((instruction / 100) % 10);
        let mode_2 = to_mode((instruction / 1_000) % 10);
        match opcode {
            1 => {
                let a = get_value(mode_1, &slice, i + 1);
                let b = get_value(mode_2, &slice, i + 2);
                let index = get_value(ParamMode::Immediate, &slice, i + 3);
                slice[index as usize] = a + b;
                i += 4;
            }
            2 => {
                let a = get_value(mode_1, &slice, i + 1);
                let b = get_value(mode_2, &slice, i + 2);
                let index = get_value(ParamMode::Immediate, &slice, i + 3);
                slice[index as usize] = a * b;
                i += 4;
            }
            3 => {
                let index = get_value(ParamMode::Immediate, &slice, i + 1);
                slice[index as usize] = receiver.next().unwrap();
                i += 2;
            }
            4 => {
                let index = get_value(mode_1, &slice, i + 1);
                latest_output = index;
                if let Some(s) = sender.as_ref() {
                    s.send(index).unwrap_or(());
                }
                i += 2;
            }
            5 => {
                let tester = get_value(mode_1, &slice, i + 1);
                let jumper = get_value(mode_2, &slice, i + 2);
                if tester != 0 {
                    i = jumper as usize;
                } else {
                    i += 3;
                }
            }
            6 => {
                let tester = get_value(mode_1, &slice, i + 1);
                let jumper = get_value(mode_2, &slice, i + 2);
                if tester == 0 {
                    i = jumper as usize;
                } else {
                    i += 3;
                }
            }
            7 => {
                let a = get_value(mode_1, &slice, i + 1);
                let b = get_value(mode_2, &slice, i + 2);
                let index = get_value(ParamMode::Immediate, &slice, i + 3);
                if a < b {
                    slice[index as usize] = 1;
                } else {
                    slice[index as usize] = 0;
                }
                i += 4;
            }
            8 => {
                let a = get_value(mode_1, &slice, i + 1);
                let b = get_value(mode_2, &slice, i + 2);
                let index = get_value(ParamMode::Immediate, &slice, i + 3);
                if a == b {
                    slice[index as usize] = 1;
                } else {
                    slice[index as usize] = 0;
                }
                i += 4;
            }
            99 => break,
            _ => panic!("Unexpected opcode: {}", opcode),
        }
    }

    latest_output
}

#[test]
fn d7_test() {
    assert_eq!(
        exercise_1(read_input("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")),
        43210
    );

    assert_eq!(
        exercise_2(read_input(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
        )),
        139629729
    );
    assert_eq!(
        exercise_2(read_input(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
        )),
        18216
    );
}

#[bench]
fn d7_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day7.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d7_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day7.txt"));
    b.iter(|| exercise_2(input.clone()));
}

#[bench]
fn d7_bench_parse(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day7.txt")));
}

