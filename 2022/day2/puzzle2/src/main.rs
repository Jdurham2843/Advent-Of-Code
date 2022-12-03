use std::cmp::Ordering;

#[derive(Debug, Eq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn value(letter_move: char) -> Move {
        match letter_move {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            other     => panic!("You supplied a bad input of {}", other)
        }
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Move::Rock, Move::Scissors) => Ordering::Greater,
            (Move::Scissors, Move::Rock) => Ordering::Less,
            (me, opp) => (*me as u32).cmp(&(*opp as u32))
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug)]
enum DesiredResult {
    Lose,
    Draw,
    Win
}

impl DesiredResult {
    fn value(letter: char) -> Self {
        match letter {
            'X' => DesiredResult::Lose,
            'Y' => DesiredResult::Draw,
            'Z' => DesiredResult::Win,
            other => panic!("You supplied a bad input of {}", letter)
        }
    }

    fn total(&self, opponent_move: Move) -> u32 {
        match self {
            DesiredResult::Lose => ((opponent_move as i32) - 1).rem_euclid(3) as u32 + 1,
            DesiredResult::Draw => ((opponent_move as u32) + 1) + 3,
            DesiredResult::Win  => (((opponent_move as u32) + 1 ).rem_euclid(3)) + 1 + 6
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Please supply rock, paper, scissors playbook");
        std::process::exit(1);
    }

    let filename = args.get(1).unwrap();
    let playbook: Vec<(Move, DesiredResult)> = parse_playbook(filename).iter()
        .map(|play| (Move::value(play.0), DesiredResult::value(play.1)))
        .collect();

    let total: u32 = playbook.iter()
        .map(|play| play.1.total(play.0))
        .sum();

    println!("{:?}", total);
}

fn parse_playbook(filename: &str) -> Vec<(char, char)> {
    std::fs::read_to_string(filename)
        .expect(format!("There was an issue reading {}", filename).as_str())
        .lines()
        .map(|line| line.trim().chars().collect())
        .map(|parts: Vec<char>| (parts[0], parts[2]))
        .collect()
}
