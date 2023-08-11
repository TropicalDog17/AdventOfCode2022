use color_eyre;
use std::str::FromStr;
#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}
#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}
impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("Not a valid move {c:?}!")),
        }
    }
}
impl FromStr for Round {
    type Err = color_eyre::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else{
            return Err(color_eyre::eyre::eyre!("Expected <theirs>SP<ours>EOF, got {s:?}"));
        };
        Ok(Self {
            theirs: theirs.try_into()?,
            ours: ours.try_into()?,
        })
    }
}
enum Outcome {
    Win,
    Draw,
    Loss,
}
impl Move {
    fn beats(&self, other: &Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors) | (Self::Scissors, Self::Paper) | (Self::Paper, Self::Rock)
        )
    }
}
impl Round {
    fn inherent_point(&self) -> u64 {
        match self.ours {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    fn outcome(&self) -> Outcome {
        if self.theirs.beats(&self.ours) {
            Outcome::Loss
        } else if self.ours.beats(&self.theirs) {
            Outcome::Win
        } else {
            Outcome::Draw
        }
    }
    fn versus_point(&self) -> u64 {
        match self.outcome() {
            Outcome::Draw => 3,
            Outcome::Loss => 0,
            Outcome::Win => 6,
        }
    }
    fn point(&self) -> u64 {
        self.inherent_point() + self.versus_point()
    }
}
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    for round in include_str!("input.txt").lines().map(|line| line.parse::<Round>()) {
        let round = round?;
        println!("{round:?}, point: {}", round.point());
    }
    let result = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<Round>())
        .map(|round| round.unwrap().point())
        .sum::<u64>();
    println!("result: {result}");
    Ok(())
}
