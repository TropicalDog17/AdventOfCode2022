use itertools::Itertools;
use std::collections::HashSet;
#[derive(Debug, Clone, Copy)]

enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    StandStill,
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
struct LongerRope {
    segments: Vec<Rope>,
    visited: Vec<Position>,
}
impl LongerRope {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn default() -> Self {
        let mut segments = Vec::new();
        for _ in 0..9 {
            segments.push(Rope::default());
        }
        Self {
            segments,
            visited: vec![Position::default()],
        }
    }
}
#[derive(Default, Clone, Debug)]
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
    // pub fn is_touching_diagonal(&self) -> bool {
    //     self.is_touching() && !self.is_same_row_or_col()
    // }
    // pub fn is_touching_adjacent(&self) -> bool {
    //     self.is_touching() && self.is_same_row_or_col()
    // }
    pub fn move_head(&mut self, direction: &Direction) {
        match direction {
            Direction::Down => self.head.row -= 1,
            Direction::Up => self.head.row += 1,
            Direction::Left => self.head.col -= 1,
            Direction::Right => self.head.col += 1,
            Direction::DownLeft => {
                self.head.row -= 1;
                self.head.col -= 1;
            }
            Direction::DownRight => {
                self.head.row -= 1;
                self.head.col += 1;
            }
            Direction::UpRight => {
                self.head.row += 1;
                self.head.col += 1;
            }
            Direction::UpLeft => {
                self.head.row += 1;
                self.head.col -= 1;
            }
            // Stand still
            Direction::StandStill => {
                self.head.row += 0;
                self.head.col += 0;
            }
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
            _ => {
                self.tail.row += 0;
                self.tail.col += 0;
            }
        }
        self.visited.push(self.tail);
    }
    // Relative position of Head to Tail
    pub fn relative_direction(&self) -> Direction {
        let col_diff = self.head.col - self.tail.col;
        let row_diff = self.head.row - self.tail.row;
        match (row_diff, col_diff) {
            (2, 0) => Direction::Up,
            (-2, 0) => Direction::Down,
            (0, 2) => Direction::Right,
            (0, -2) => Direction::Left,
            (2, 1) | (1, 2) => Direction::UpRight,
            (2, -1) | (1, -2) => Direction::UpLeft,
            (-2, 1) | (-1, 2) => Direction::DownRight,
            (-2, -1) | (-1, -2) => Direction::DownLeft,
            _ => unimplemented!(),
        }
    }
    pub fn get_direction(&mut self) -> Direction {
        if self.is_overlapping() || self.is_touching() {
            return Direction::StandStill;
        }
        self.relative_direction()
    }
    // Move one unit{ row: 0, col: 0 }
    // pub fn move_rope(&mut self, direction: &Direction) {
    //     // dbg!(self.head, self.tail);
    //     self.move_head(direction);

    //     // Move both head and tail based on current state
    //     if self.is_overlapping() || self.is_touching() {
    //         return;
    //     }
    //     self.move_tail(&self.relative_direction());
    // }
    // pub fn move_direction(&mut self, direction: &Direction, distance: usize) {
    //     for _ in 0..distance {
    //         self.move_rope(direction);
    //     }
    // }
}
impl LongerRope {
    pub fn move_head(&mut self, direction: &Direction) {
        let mut next_direction = *direction;
        for segment in &mut self.segments {
            segment.move_head(&next_direction);
            next_direction = segment.get_direction();
            segment.move_tail(&next_direction);
        }
        // dbg!(self.segments.last().unwrap().tail);
        self.visited.push(self.segments.last().unwrap().tail);
        // Move both head and tail based on current state
    }
    pub fn move_direction(&mut self, direction: &Direction, distance: usize) {
        for _ in 0..distance {
            for i in 0..9 {
                dbg!(self.segments.get(i).unwrap().head);
                dbg!(self.segments.get(i).unwrap().tail);
            }
            self.move_head(direction);
        }
    }
    pub fn get_unique_visited_count(&self) -> usize {
        let unique_positions = self.visited.iter().unique().collect::<HashSet<_>>();
        unique_positions.len()
    }
}
fn main() {
    let lines = include_str!("input.txt").lines();
    let mut longer_rope = LongerRope::new();
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
        longer_rope.move_direction(direction, distance);
    }
    // print unique count of visited positions

    // println!("count: {}", longer_rope.get_unique_visited_count());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_new_longer_rope() {
        let mut longer_rope = LongerRope::new();
        let lines = include_str!("input.txt").lines();
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
            longer_rope.move_direction(direction, distance)
        }
        assert_eq!(longer_rope.get_unique_visited_count(), 36);
    }
}
