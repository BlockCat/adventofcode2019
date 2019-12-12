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

    pub fn manhattan(lhs: &Self, other: &Self) -> usize {
        ((lhs.0 - other.0).abs() + (lhs.1 - other.1).abs()) as usize
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

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Vector3(pub isize, pub isize, pub isize);

impl Vector3 {
    pub fn sign(self) -> Vector3 {
        Vector3(self.0.signum(), self.1.signum(), self.2.signum())
    }

    pub fn manhattan(lhs: &Self, other: &Self) -> usize {
        ((lhs.0 - other.0).abs() + (lhs.1 - other.1).abs() + (lhs.2 - other.2).abs()) as usize
    }

    pub fn length_sq(&self) -> usize {
        (self.0.abs() + self.1.abs() + self.2.abs()) as usize
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vector3 {    

    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
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
    pub width: usize,
    pub height: usize,
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

    pub fn from_vec(grid: Vec<Vec<T>>) -> Grid<T> {
        let width = grid[0].len();
        let height = grid.len();
        Grid {
            grid: grid.into_iter().flatten().collect(),
            width,
            height,
        }
    }

    pub fn to_vec(&self) -> Vec<Vec<T>> {
        self.grid.chunks(self.width).map(|x| Vec::from(x)).collect()
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