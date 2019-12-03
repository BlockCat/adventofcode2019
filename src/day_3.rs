use utils::*;

use crate::test::Bencher;

type Wire = Vec<(Direction, u32)>;

#[derive(Clone)]
struct WireInstruction {
    position: Vector2,
    wire: Wire,
    index: usize,
    dist: u32,
}

impl WireInstruction {
    fn new(wire: Wire) -> WireInstruction {
        WireInstruction {
            position: Vector2(0, 0),
            index: 0,
            dist: wire[0].1,
            wire: wire,
        }
    }
}

impl Iterator for WireInstruction {
    type Item = Vector2;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.dist == 0) {
            self.index += 1;
            if self.index == self.wire.len() {
                return None;
            } else {
                self.dist = self.wire[self.index].1;
            }
        }
        self.position += self.wire[self.index].0;
        self.dist -= 1;

        Some(self.position)
    }
}

#[test]
pub fn run() {
    let mut input = read_input(include_str!("input/day3.txt"));
    println!("{}", exercise_1(input.clone()));
    println!("{}", exercise_2(input.clone()));
}

fn exercise_1((wire_1, wire_2): (WireInstruction, WireInstruction)) -> usize {
    let mut grid = InfiniteGrid::new();
    let mut closest_distance = std::usize::MAX;

    let origin = Vector2(0, 0);

    for pos_1 in wire_1 {
        grid.set(pos_1.0, pos_1.1, true);
    }

    for pos_2 in wire_2 {
        if Vector2::manhattan(&pos_2, &origin) < closest_distance
            && grid.get(pos_2.0, pos_2.1) == Some(&true)
        {
            closest_distance = Vector2::manhattan(&pos_2, &origin);
        }
    }

    closest_distance
}

fn exercise_2((wire_1, wire_2): (WireInstruction, WireInstruction)) -> usize {
    let mut grid = InfiniteGrid::new();

    let mut closest_distance = std::usize::MAX;
    let origin = Vector2(0, 0);

    for (index, pos_1) in wire_1.enumerate() {
        if let Some((_, false)) = grid.get(pos_1.0, pos_1.1){
            grid.set(pos_1.0, pos_1.1, (index + 1, true));
        }
    }

    for (index, pos_2) in wire_2.enumerate() {
        let result = grid.get(pos_2.0, pos_2.1);        
        if let Some(&(steps_o_1, true)) = result {
            if steps_o_1 + index + 1 < closest_distance {
                closest_distance = steps_o_1 + index + 1;
            }
        }
    }

    closest_distance
}

fn read_input(input: &str) -> (WireInstruction, WireInstruction) {
    let mut it = input.lines();
    let wire_1 = read_line(it.next().unwrap());
    let wire_2 = read_line(it.next().unwrap());

    let wire_1 = WireInstruction::new(wire_1);
    let wire_2 = WireInstruction::new(wire_2);

    (wire_1, wire_2)
}

fn read_line(input: &str) -> Vec<(Direction, u32)> {
    input
        .split(',')
        .map(|x| {
            let dir = match &x[0..1] {
                "U" => Direction::North,
                "D" => Direction::South,
                "L" => Direction::West,
                "R" => Direction::East,
                _ => panic!(),
            };
            let distance = x[1..].parse::<u32>().unwrap();

            (dir, distance)
        })
        .collect()
}

#[test]
fn d3_test() {
    let input = r"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
    assert_eq!(exercise_1(read_input(input)), 159);
    assert_eq!(exercise_2(read_input(input)), 610);
    let input = r"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    assert_eq!(exercise_1(read_input(input)), 135);
    assert_eq!(exercise_2(read_input(input)), 410);
    
    assert_eq!(exercise_1(read_input(include_str!("input/day3.txt"))), 293);
    assert_eq!(exercise_2(read_input(include_str!("input/day3.txt"))), 27306);
}

#[bench]
fn d2_bench_parse(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day3.txt")));
}
#[bench]
fn d2_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day3.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d2_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day3.txt"));
    b.iter(|| exercise_2(input.clone()));
}

