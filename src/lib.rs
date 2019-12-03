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

#[derive(PartialEq, Eq, Clone)]
pub struct InfiniteGrid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    grid: HashMap<(isize, isize), Grid<T>>,
    cache: Option<(isize, isize, *mut Grid<T>)>,
    width: usize,
    height: usize,
}

impl<T> InfiniteGrid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    pub fn new() -> InfiniteGrid<T> {
        InfiniteGrid {
            grid: HashMap::new(),
            cache: None,
            width: 100,
            height: 100,
        }
    }

    fn get_grid(&mut self, x: isize, y: isize) -> *mut Grid<T> {
        if !self.grid.contains_key(&(x, y)) {
            self.grid.insert((x, y), Grid::new(self.width, self.height));
        }
        self.grid.get_mut(&(x, y)).unwrap()
    }

    pub fn set(&mut self, x: isize, y: isize, value: T) {
        let cx = x / self.width as isize;
        let cy = y / self.height as isize;
        let x = (x % self.width as isize).abs() as usize;
        let y = (y % self.height as isize).abs() as usize;
        if self.cache == None {
            self.cache = Some((cx, cy, self.get_grid(cx, cy)));
        } else if let Some((ocx, ocy, _)) = self.cache {
            if ocx != cx && ocy != cy {
                self.cache = Some((cx, cy, self.get_grid(cx, cy)));
            }
        }
        //self.cache = Some((cx, cy, self.get_grid(cx, cy)));
        unsafe {
            (*self.cache.unwrap().2).set(x, y, value);
        }
    }

    pub fn get(&mut self, mx: isize, my: isize) -> Option<&T> {
        let w = self.width as isize;
        let h = self.height as isize;

        let cx = if mx < 0 { mx / w - 1 } else { mx / w };
        let cy = if my < 0 { my / h - 1 } else { my / h };

        let x = (mx - cx * w) as usize;
        let y = (my - cy * h) as usize;

        println!("({},{}) -> [({}, {}) sub ({}, {})]", mx, my, cx, cy, x, y);
        if self.cache == None {
            self.cache = Some((cx, cy, self.get_grid(cx, cy)));
        } else if let Some((ocx, ocy, _)) = self.cache {
            if ocx != cx && ocy != cx {
                self.cache = Some((cx, cy, self.get_grid(cx, cy)));
            }
        }
        //self.cache = Some((cx, cy, self.get_grid(cx, cy)));
        unsafe { (*self.cache.unwrap().2).get(x, y) }
    }
}
