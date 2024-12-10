#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A location on a (rows, cols) 2D grid.
/// `Loc(0, 0)` is top left;
/// `Loc(0, 1)` is to the right (East) of `Loc(0, 0)`;
/// `Loc(1, 0)` is below (South) `Loc(0, 0)`;
pub struct Loc(pub isize, pub isize);

impl std::ops::Add for Loc {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Loc(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Add<Dir> for Loc {
    type Output = Self;
    fn add(self, rhs: Dir) -> Self {
        self + Loc::from(rhs)
    }
}

impl std::ops::Sub for Loc {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Loc(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Mul<isize> for Loc {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self {
        Loc(self.0 * rhs, self.1 * rhs)
    }
}

impl Loc {
    pub fn in_bounds(&self, bounds: (usize, usize)) -> bool {
        self.0 >= 0 && self.1 >= 0 && self.0 < bounds.0 as isize && self.1 < bounds.1 as isize
    }
}

impl From<(isize, isize)> for Loc {
    fn from(val: (isize, isize)) -> Self {
        Loc(val.0, val.1)
    }
}

impl From<(usize, usize)> for Loc {
    fn from(val: (usize, usize)) -> Self {
        Loc(val.0 as isize, val.1 as isize)
    }
}

impl From<Loc> for (usize, usize) {
    fn from(val: Loc) -> Self {
        (val.0 as usize, val.1 as usize)
    }
}

impl From<Dir> for Loc {
    fn from(val: Dir) -> Self {
        match val {
            Dir::North => Loc(-1, 0),
            Dir::NorthEast => Loc(-1, 1),
            Dir::East => Loc(0, 1),
            Dir::SouthEast => Loc(1, 1),
            Dir::South => Loc(1, 0),
            Dir::SouthWest => Loc(1, -1),
            Dir::West => Loc(0, -1),
            Dir::NorthWest => Loc(-1, -1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl std::ops::Mul<isize> for Dir {
    type Output = Loc;
    fn mul(self, rhs: isize) -> Loc {
        Loc::from(self) * rhs
    }
}

pub const DIR4: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];
pub const DIR8: [Dir; 8] = [
    Dir::North,
    Dir::NorthEast,
    Dir::East,
    Dir::SouthEast,
    Dir::South,
    Dir::SouthWest,
    Dir::West,
    Dir::NorthWest,
];
