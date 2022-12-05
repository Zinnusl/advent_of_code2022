type Score = i32;

#[derive(Debug, Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn value(&self) -> Score {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    pub fn beats(&self, other: &Shape) -> MatchResult {
        let diff = self.value() - other.value();
        match diff {
            0 => MatchResult::Draw,
            1 | -2 => MatchResult::Win,
            _ => MatchResult::Lose,
        }
    }
}

impl From<&str> for Shape {
    fn from(c: &str) -> Shape {
        match c {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("Invalid shape"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum MatchResult {
    Win,
    Draw,
    Lose,
}

impl MatchResult {
    pub fn value(&self) -> Score {
        match self {
            MatchResult::Win => 6,
            MatchResult::Draw => 3,
            MatchResult::Lose => 0,
        }
    }
}

impl From<&str> for MatchResult {
    fn from(c: &str) -> MatchResult {
        match c {
            "Z" => MatchResult::Win,
            "Y" => MatchResult::Draw,
            "X" => MatchResult::Lose,
            _ => panic!("Invalid match result"),
        }
    }
}

impl From<(&Shape, &MatchResult)> for Shape {
    fn from(t: (&Shape, &MatchResult)) -> Shape {
        match t {
            (Shape::Rock, MatchResult::Win) => Shape::Paper,
            (Shape::Paper, MatchResult::Win) => Shape::Scissors,
            (Shape::Scissors, MatchResult::Win) => Shape::Rock,
            (Shape::Rock, MatchResult::Draw) => Shape::Rock,
            (Shape::Paper, MatchResult::Draw) => Shape::Paper,
            (Shape::Scissors, MatchResult::Draw) => Shape::Scissors,
            (Shape::Rock, MatchResult::Lose) => Shape::Scissors,
            (Shape::Paper, MatchResult::Lose) => Shape::Rock,
            (Shape::Scissors, MatchResult::Lose) => Shape::Paper,
        }
    }
}

fn main() {
    let matches = std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .lines()
        .map(|line| {
            let mut line_matches = line.matches(&['A', 'B', 'C', 'X', 'Y', 'Z']);
            (
                Shape::from(line_matches.next().unwrap()),
                MatchResult::from(line_matches.next().unwrap()),
            )
        })
        .collect::<Vec<_>>();

    let total_score = matches
        .iter()
        .map(|(enemy, result)| result.value() + Shape::from((enemy, result)).value())
        .sum::<Score>();

    println!("Total score => {}", total_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rock_beats_scissors() {
        assert_eq!(Shape::Rock.beats(&Shape::Scissors), MatchResult::Win);
    }

    #[test]
    fn test_scissors_beats_paper() {
        assert_eq!(Shape::Scissors.beats(&Shape::Paper), MatchResult::Win);
    }

    #[test]
    fn test_paper_beats_rock() {
        assert_eq!(Shape::Paper.beats(&Shape::Rock), MatchResult::Win);
    }

    #[test]
    fn test_rock_loses_to_paper() {
        assert_eq!(Shape::Rock.beats(&Shape::Paper), MatchResult::Lose);
    }

    #[test]
    fn test_scissors_loses_to_rock() {
        assert_eq!(Shape::Scissors.beats(&Shape::Rock), MatchResult::Lose);
    }

    #[test]
    fn test_paper_loses_to_scissors() {
        assert_eq!(Shape::Paper.beats(&Shape::Scissors), MatchResult::Lose);
    }

    #[test]
    fn test_rock_draws_with_rock() {
        assert_eq!(Shape::Rock.beats(&Shape::Rock), MatchResult::Draw);
    }

    #[test]
    fn test_scissors_draws_with_scissors() {
        assert_eq!(Shape::Scissors.beats(&Shape::Scissors), MatchResult::Draw);
    }

    #[test]
    fn test_paper_draws_with_paper() {
        assert_eq!(Shape::Paper.beats(&Shape::Paper), MatchResult::Draw);
    }
}
