use itertools::Itertools;
use std::collections::HashSet;
#[derive(Debug)]

enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}
#[derive(Default, PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    row: i16,
    col: i16,
}
impl Position {
    pub fn new(row: i16, col: i16) -> Self {
        Self { row, col }
    }
}
struct Rope {
    head: Position,
    tail: Position,
    visited: Vec<Position>,
}
impl Rope {
    pub fn new() -> Self {
        let mut visited = vec![Position::default()];
        Self {
            head: Position::default(),
            tail: Position::default(),
            visited,
        }
    }
    pub fn is_touching(&self) -> bool {
        let col_diff = self.head.col - self.tail.col;
        let row_diff = self.head.row - self.tail.row;
        if (-1..2).contains(&col_diff) && (-1..2).contains(&row_diff) {
            return true;
        }
        false
    }
    pub fn is_same_row_or_col(&self) -> bool {
        let col_diff = self.head.col - self.tail.col;
        let row_diff = self.head.row - self.tail.row;
        col_diff == 0 || row_diff == 0
    }
    pub fn is_overlapping(&self) -> bool {
        let col_diff = self.head.col - self.tail.col;
        let row_diff = self.head.row - self.tail.row;
        col_diff == 0 && row_diff == 0
    }
    pub fn is_touching_diagonal(&self) -> bool {
        self.is_touching() && !self.is_same_row_or_col()
    }
    pub fn is_touching_adjacent(&self) -> bool {
        self.is_touching() && self.is_same_row_or_col()
    }
    pub fn move_head(&mut self, direction: &Direction) {
        match direction {
            Direction::Down => self.head.row -= 1,
            Direction::Up => self.head.row += 1,
            Direction::Left => self.head.col -= 1,
            Direction::Right => self.head.col += 1,
            _ => unimplemented!(),
        }
    }
    pub fn move_tail(&mut self, direction: &Direction) {
        match direction {
            Direction::Down => self.tail.row -= 1,
            Direction::Up => self.tail.row += 1,
            Direction::Left => self.tail.col -= 1,
            Direction::Right => self.tail.col += 1,
            Direction::DownLeft => {
                self.tail.row -= 1;
                self.tail.col -= 1;
            }
            Direction::DownRight => {
                self.tail.row -= 1;
                self.tail.col += 1;
            }
            Direction::UpRight => {
                self.tail.row += 1;
                self.tail.col += 1;
            }
            Direction::UpLeft => {
                self.tail.row += 1;
                self.tail.col -= 1;
            }
        }
        self.visited.push(self.tail);
    }
    // Relative position of Head based on Tail
    pub fn relative_direction(&self) -> Direction {
        let col_diff = self.head.col - self.tail.col;
        let row_diff = self.head.row - self.tail.row;
        match (row_diff > 0, col_diff > 0) {
            (true, true) => Direction::UpRight,
            (true, false) => Direction::UpLeft,
            (false, true) => Direction::DownRight,
            (false, false) => Direction::DownLeft,
        }
    }
    // Move one unit{ row: 0, col: 0 }
    pub fn move_rope(&mut self, direction: &Direction) {
        dbg!(self.head, self.tail);

        // Move both head and tail based on current state
        if self.is_overlapping() {
            self.move_head(direction);
            return;
        }
        if self.is_touching_adjacent() {
            self.move_head(direction);
            if self.is_overlapping() || self.is_touching_diagonal() {
                return;
            }
            // Move tail according to plank length
            self.move_tail(direction);
            return;
        }
        if self.is_touching_diagonal() {
            self.move_head(direction);
            if !self.is_touching() {
                dbg!(self.relative_direction());
                self.move_tail(&self.relative_direction());
            }
        }
    }
    pub fn move_direction(&mut self, direction: &Direction, distance: usize) {
        for _ in 0..distance {
            self.move_rope(direction);
        }
    }
}

fn main() {
    let lines = include_str!("input.txt").lines();
    let mut rope = Rope::new();
    for line in lines {
        let parsed_args = line.split_whitespace().collect::<Vec<_>>();
        let direction = match parsed_args[0] {
            "R" => &Direction::Right,
            "L" => &Direction::Left,
            "U" => &Direction::Up,
            "D" => &Direction::Down,
            _ => unimplemented!(),
        };
        let distance = parsed_args[1].parse::<usize>().unwrap();
        rope.move_direction(direction, distance)
    }

    // print unique count of visited positions
    let unique_positions = rope.visited.iter().unique().collect::<HashSet<_>>();
    println!("Unique positions: {:?}", rope.visited);
    println!("count: {}", unique_positions.len());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_touching_diagonal() {
        let mut rope = Rope::new();
        rope.head = Position::new(1, 4);
        rope.tail = Position::new(0, 3);
        assert!(rope.is_touching_diagonal());
        assert!(!rope.is_touching_adjacent());
    }
}
