use hashbrown::HashMap;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Vector2(pub isize, pub isize);

impl Vector2 {
    pub fn length_sq(&self) -> isize {
        self.0 * self.0 + self.1 * self.1
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector2(self.0 - other.0, self.1 - other.1)
    }
}

impl Add<Direction> for Vector2 {
    type Output = Self;

    fn add(self, other: Direction) -> Self {
        self + match other {
            Direction::North => Vector2(0, -1),
            Direction::East => Vector2(1, 0),
            Direction::South => Vector2(0, 1),
            Direction::West => Vector2(-1, 0),
        }
    }
}

impl AddAssign<Direction> for Vector2 {
    fn add_assign(&mut self, other: Direction) {
        *self = *self
            + match other {
                Direction::North => Vector2(0, -1),
                Direction::East => Vector2(1, 0),
                Direction::South => Vector2(0, 1),
                Direction::West => Vector2(-1, 0),
            }
    }
}

impl Sub<Direction> for Vector2 {
    type Output = Self;

    fn sub(self, other: Direction) -> Self {
        self - match other {
            Direction::North => Vector2(0, -1),
            Direction::East => Vector2(1, 0),
            Direction::South => Vector2(0, 1),
            Direction::West => Vector2(-1, 0),
        }
    }
}

impl SubAssign<Direction> for Vector2 {
    fn sub_assign(&mut self, other: Direction) {
        *self = *self
            - match other {
                Direction::North => Vector2(0, -1),
                Direction::East => Vector2(1, 0),
                Direction::South => Vector2(0, 1),
                Direction::West => Vector2(-1, 0),
            }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
    pub fn right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    pub fn reverse(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Grid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    grid: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    pub fn new(width: usize, height: usize) -> Grid<T> {
        Grid {
            grid: vec![Default::default(); width * height],
            width,
            height,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.grid[x + y * self.width] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.grid.get(x + y * self.width)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.grid.get_mut(x + y * self.width)
    }
}