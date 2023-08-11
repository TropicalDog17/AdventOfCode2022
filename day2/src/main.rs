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
struct Strategy {
    theirs: Move,
    expected: Outcome,
}
impl From<Strategy> for Round {
    fn from(s: Strategy) -> Self {
        let move_to_play: Move;
        match s.expected {
            Outcome::Draw => move_to_play = s.theirs,
            Outcome::Win => match s.theirs {
                Move::Paper => move_to_play = Move::Scissors,
                Move::Rock => move_to_play = Move::Paper,
                Move::Scissors => move_to_play = Move::Rock,
            },
            Outcome::Loss => match s.theirs {
                Move::Paper => move_to_play = Move::Rock,
                Move::Rock => move_to_play = Move::Scissors,
                Move::Scissors => move_to_play = Move::Paper,
            },
        }
        Round {
            theirs: s.theirs,
            ours: move_to_play,
        }
    }
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
impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(color_eyre::eyre::eyre!("Not a valid outcome {c:?}!")),
        }
    }
}
impl FromStr for Strategy {
    type Err = color_eyre::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(expected), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else{
            return Err(color_eyre::eyre::eyre!("Expected <theirs>SP<outcome>EOF, got {s:?}"));
        };
        Ok(Self {
            theirs: theirs.try_into()?,
            expected: expected.try_into()?,
        })
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

    let result = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<Strategy>())
        .map(|strategy| strategy.unwrap())
        .map(|strategy| Round::from(strategy).point())
        .sum::<u64>();
    println!("result: {result}");
    Ok(())
}
