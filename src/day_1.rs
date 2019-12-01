use crate::test::Bencher;

#[test]
fn run() {
    let input = read_input(include_str!("input/day1.txt"));
    println!("{}", exercise_1(&input));
    println!("{}", exercise_2(&input));
}

pub fn exercise_1(slice: &Vec<i32>) -> i32 {
    slice.iter().map(|x| x / 3 - 2).sum()
}

pub fn exercise_2(slice: &Vec<i32>) -> i32 {
    slice.iter().map(calculate_fuel).sum()
}

fn calculate_fuel(mass: &i32) -> i32 {
    let mut fuel = *mass;
    let mut sum = fuel / 3 - 2;
    while fuel > 0 {
        sum += fuel;
        fuel = fuel / 3 - 2;
    }
    sum
}

pub fn read_input(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse::<i32>().unwrap()).collect()
}

#[test]
fn d1_test() {
    assert_eq!(exercise_1(&vec!(12)), 2);
    assert_eq!(exercise_1(&vec!(14)), 2);
    assert_eq!(exercise_1(&vec!(1969)), 654);
    assert_eq!(exercise_1(&vec!(100756)), 33583);
    assert_eq!(exercise_2(&vec!(12)), 2);
    assert_eq!(exercise_2(&vec!(1969)), 966);
    assert_eq!(exercise_2(&vec!(100756)), 50346);
}

#[bench]
fn d1_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day1.txt"));
    b.iter(|| exercise_1(&input));
}

#[bench]
fn d1_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day1.txt"));
    b.iter(|| exercise_2(&input));
}
