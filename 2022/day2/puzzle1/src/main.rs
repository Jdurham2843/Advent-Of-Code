use std::cmp::Ordering;

#[derive(Debug, Eq, Clone, Copy)]
enum Move {
    Rock     = 1,
    Paper    = 2,
    Scissors = 3
}

impl Move {
    fn value(letter_move: char) -> Move {
        match letter_move {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
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

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Please supply rock, paper, scissors playbook");
        std::process::exit(1);
    }

    let filename = args.get(1).unwrap();
    let playbook: Vec<(Move, Move)> = parse_playbook(filename).iter()
        .map(|play| (Move::value(play.0), Move::value(play.1)))
        .collect();

    let total: u32 = playbook.iter()
        .map(|play| calculate(play))
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

fn calculate((opponent_move, my_move): &(Move, Move)) -> u32 {
    match my_move.cmp(&opponent_move) {
        Ordering::Greater => *my_move as u32 + 6,
        Ordering::Equal   => *my_move as u32 + 3,
        Ordering::Less    => *my_move as u32
    }
}
