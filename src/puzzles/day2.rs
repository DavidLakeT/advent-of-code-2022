use std::io::{Error, ErrorKind};

struct Round {
    theirs: Selection,
    ours: Selection,
}

impl Round {
    pub fn parse_str_part1(line: &str) -> Result<Self, Error> {
        let mut chars = line.chars();

        let (theirs, _, ours) = (chars.next(), chars.next(), chars.next());

        match (theirs, ours) {
            (Some(theirs), Some(ours)) => Ok(Self {
                theirs: Selection::parse(theirs).unwrap(),
                ours: Selection::parse(ours).unwrap(),
            }),

            _ => Err(Error::new(ErrorKind::Other, "Could not parse round info.")),
        }
    }

    pub fn parse_str_part2(line: &str) -> Result<Self, Error> {
        let mut chars = line.chars();

        let (theirs, _, ours) = (chars.next(), chars.next(), chars.next());

        match (theirs, ours) {
            (Some(theirs), Some(ours)) => {
                let theirs = Selection::parse(theirs).unwrap();
                let ours = theirs.counterpick(Outcome::parse_expected_oc(ours).unwrap());

                Ok(Self { theirs, ours })
            }

            _ => Err(Error::new(ErrorKind::Other, "Could not parse round info.")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Selection {
    Rock,
    Paper,
    Scissors,
}

impl Selection {
    pub fn parse(ch: char) -> Result<Self, Error> {
        match ch {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(Error::new(ErrorKind::Other, "Could not parse round info.")),
        }
    }

    fn outcome(self, theirs: Selection) -> Outcome {
        match (self, theirs) {
            (Selection::Rock, Selection::Rock) => Outcome::Draw,
            (Selection::Rock, Selection::Paper) => Outcome::Loss,
            (Selection::Rock, Selection::Scissors) => Outcome::Win,
            (Selection::Paper, Selection::Rock) => Outcome::Win,
            (Selection::Paper, Selection::Paper) => Outcome::Draw,
            (Selection::Paper, Selection::Scissors) => Outcome::Loss,
            (Selection::Scissors, Selection::Rock) => Outcome::Loss,
            (Selection::Scissors, Selection::Paper) => Outcome::Win,
            (Selection::Scissors, Selection::Scissors) => Outcome::Draw,
        }
    }

    fn selection_points(&self) -> u64 {
        match self {
            Selection::Rock => 1,
            Selection::Paper => 2,
            Selection::Scissors => 3,
        }
    }

    fn counterpick(self, expected_oc: Outcome) -> Self {
        match (self, expected_oc) {
            (Self::Rock, Outcome::Win) => Self::Paper,
            (Self::Rock, Outcome::Draw) => Self::Rock,
            (Self::Rock, Outcome::Loss) => Self::Scissors,
            (Self::Paper, Outcome::Win) => Self::Scissors,
            (Self::Paper, Outcome::Draw) => Self::Paper,
            (Self::Paper, Outcome::Loss) => Self::Rock,
            (Self::Scissors, Outcome::Win) => Self::Rock,
            (Self::Scissors, Outcome::Draw) => Self::Scissors,
            (Self::Scissors, Outcome::Loss) => Self::Paper,
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn earned_points(self) -> u64 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }

    pub fn parse_expected_oc(ch: char) -> Result<Self, Error> {
        match ch {
            'X' => Ok(Self::Loss),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(Error::new(
                ErrorKind::Other,
                "Could not parse expected outcome info.",
            )),
        }
    }
}

pub fn solve_part1() -> u64 {
    let mut points = 0;

    for line in include_str!(
        r"C:\Users\david\Desktop\Programacion\Rust\advent-of-code-2022\inputs\day2.txt"
    )
    .lines()
    {
        if let Ok(round) = Round::parse_str_part1(line) {
            let theirs = round.theirs;
            let ours = round.ours;

            let earned_points = ours.selection_points() + ours.outcome(theirs).earned_points();
            points += earned_points;
        }
    }

    points
}

pub fn solve_part2() -> u64 {
    let mut points = 0;

    for line in include_str!(
        r"C:\Users\david\Desktop\Programacion\Rust\advent-of-code-2022\inputs\day2.txt"
    )
    .lines()
    {
        if let Ok(round) = Round::parse_str_part2(line) {
            let theirs = round.theirs;
            let ours = round.ours;

            let earned_points = ours.selection_points() + ours.outcome(theirs).earned_points();
            points += earned_points;
        }
    }

    points
}
