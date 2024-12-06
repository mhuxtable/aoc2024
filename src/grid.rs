use std::fmt::Display;

use bitflags::bitflags;

pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new(default: T, width: usize, height: usize) -> Self {
        let data = vec![default; width * height];

        Self {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }

        self.data.get(y * self.width + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.width || y >= self.height {
            return None;
        }

        self.data.get_mut(y * self.width + x)
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }

    pub fn neighbours_iter_basis(&self, &(x, y): &(usize, usize)) -> GridNeighbourIterator<T> {
        self.neighbours_iter(&(x, y), NeighbourTypes::BasisSet)
    }

    pub fn neighbours_iter(
        &self,
        &(x, y): &(usize, usize),
        include: NeighbourTypes,
    ) -> GridNeighbourIterator<T> {
        assert!(x < self.width && y < self.height);

        GridNeighbourIterator {
            grid: self,
            x,
            y,
            index: 0,
            include,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x >= self.width || y >= self.height {
            panic!("out of range");
        }

        self.data[y * self.width + x] = value;
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

bitflags! {
    pub struct NeighbourTypes: u8 {
        const Horizontal = 0x1;
        const Vertical = 0x2;
        const Diagonals = 0x4;
        const BasisSet = Self::Horizontal.bits() | Self::Vertical.bits();
    }
}

impl<T> Grid<T>
where
    T: Clone + Default + From<char>,
{
    // parse assumes the grid is rectangular with constant width
    pub fn parse<Input: AsRef<str>>(input: Input) -> Result<Self, String> {
        Self::parse_with_parser(Default::default(), input, |character| character.into())
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn parse_with_parser<Input: AsRef<str>, Parser: Fn(char) -> T>(
        default: T,
        input: Input,
        parser: Parser,
    ) -> Result<Self, String> {
        let (width, height) =
            input
                .as_ref()
                .lines()
                .try_fold((None, 0), |(width, height), line| match width {
                    Some(x) if x != line.len() => Err("grid is not rectangular"),
                    _ => Ok((Some(line.len()), height + 1)),
                })?;

        if width.is_none() {
            return Err("grid is empty".to_string());
        }

        let mut grid = Self::new(default, width.unwrap(), height);

        for (y, line) in input.as_ref().lines().enumerate() {
            for (x, character) in line.chars().enumerate() {
                grid.set(x, y, parser(character));
            }
        }

        Ok(grid)
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: Clone + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ((x, _), value) in self {
            write!(f, "{}", value)?;

            if x == self.width - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl<T> Grid<T>
where
    T: Clone + Display,
{
    pub fn fmt_with_overrides<'a, F: Fn(&(usize, usize)) -> Option<char> + 'a>(
        &'a self,
        overrides: F,
    ) -> OverriddenFormatter<'a, T, F> {
        OverriddenFormatter {
            grid: self,
            overrides,
        }
    }
}

pub struct OverriddenFormatter<'a, T, F: Fn(&(usize, usize)) -> Option<char>> {
    grid: &'a Grid<T>,
    overrides: F,
}

impl<T, F> Display for OverriddenFormatter<'_, T, F>
where
    T: Clone + Display,
    F: Fn(&(usize, usize)) -> Option<char>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ((x, y), value) in self.grid {
            if let Some(override_char) = (self.overrides)(&(x, y)) {
                write!(f, "{}", override_char)?;
            } else {
                write!(f, "{}", value)?;
            }

            if x == self.grid.width - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<T> Iterator for GridIterator<'_, T>
where
    T: Clone,
{
    type Item = ((usize, usize), T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height() {
            return None;
        }

        let value = self.grid.get(self.x, self.y).unwrap().clone();
        let position = (self.x, self.y);

        self.x += 1;

        if self.x >= self.grid.width() {
            self.x = 0;
            self.y += 1;
        }

        Some((position, value))
    }
}

pub struct GridIntoIterator<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<T> Iterator for GridIntoIterator<'_, T>
where
    T: Clone,
{
    type Item = ((usize, usize), T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height() {
            return None;
        }

        let value = self.grid.get(self.x, self.y).unwrap().clone();
        let position = (self.x, self.y);

        self.x += 1;

        if self.x >= self.grid.width() {
            self.x = 0;
            self.y += 1;
        }

        Some((position, value))
    }
}

impl<'a, T> IntoIterator for &'a Grid<T>
where
    T: Clone,
{
    type Item = ((usize, usize), T);
    type IntoIter = GridIntoIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIntoIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

pub struct GridNeighbourIterator<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
    index: usize,
    include: NeighbourTypes,
}

static NEIGHBOURS: [(isize, isize); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

impl<'a, T> Iterator for GridNeighbourIterator<'a, T>
where
    T: Clone,
{
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= NEIGHBOURS.len() {
            return None;
        }
        if !self.include.contains(NeighbourTypes::Horizontal) && self.index < 2 {
            self.index += 1;
            return self.next();
        }
        if !self.include.contains(NeighbourTypes::Vertical) && self.index >= 2 && self.index < 4 {
            self.index += 1;
            return self.next();
        }
        if !self.include.contains(NeighbourTypes::Diagonals) && self.index >= 4 {
            self.index += 1;
            return self.next();
        }

        let (dx, dy) = NEIGHBOURS[self.index];
        let x = self.x as isize + dx;
        let y = self.y as isize + dy;

        if x < 0 || y < 0 || x >= self.grid.width as isize || y >= self.grid.height as isize {
            self.index += 1;
            return self.next();
        }

        let v = self.grid.get(x as usize, y as usize).unwrap();
        self.index += 1;

        Some(((x as usize, y as usize), v))
    }
}
