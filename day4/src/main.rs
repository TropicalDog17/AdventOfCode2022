use color_eyre;
use itertools::Itertools;
use std::str::FromStr;
#[derive(Debug)]
struct Assignment {
    begin: usize,
    end: usize,
}
#[derive(Debug)]
struct Pair {
    first: Assignment,
    second: Assignment,
}
impl FromStr for Assignment {
    type Err = color_eyre::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (begin, end) = s
            .split('-')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Self { begin, end })
    }
}
impl FromStr for Pair {
    type Err = color_eyre::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s
            .split(',')
            .map(|s| s.parse::<Assignment>().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Self { first, second })
    }
}
impl Pair {
    fn is_fully_contained(&self) -> bool {
        if (self.first.begin >= self.second.begin && self.first.end <= self.second.end)
            || (self.first.begin <= self.second.begin && self.first.end >= self.second.end)
        {
            return true;
        }
        false
    }
    fn is_overlapped(&self) -> bool {
        if self.is_fully_contained()
            || (self.first.begin <= self.second.begin && self.first.end >= self.second.begin)
            || (self.first.begin <= self.second.end && self.first.end >= self.second.end)
        {
            return true;
        }
        false
    }
}
fn main() -> color_eyre::Result<()> {
    let count = include_str!("input.txt")
        .lines()
        .flat_map(|s| s.parse::<Pair>())
        .filter(|pair| pair.is_overlapped())
        .count();
    println!("{count}");
    Ok(())
}
