#[derive(Debug)]
struct Assignment {
    start: u32,
    end:   u32
}

impl Assignment {
    fn create(assignment_string: &str) -> Self {
        let parts: Vec<&str> = assignment_string.split("-").collect();

        Assignment { 
            start: u32::from_str_radix(parts[0], 10).unwrap(),
            end:   u32::from_str_radix(parts[1], 10).unwrap()
        }
    }

    fn either_full_contained(&self, other: &Self) -> bool {
        let self_contains_other = (other.start >= self.start && other.start <= self.end) &&
            (other.end <= self.end && other.end >= self.start);
        let other_contains_self = (self.start >= other.start && self.start <= other.end) &&
        (self.end <= other.end && self.end >= other.start);

        self_contains_other || other_contains_self
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Please supply an input file");
        std::process::exit(1);
    }

    let filename = args.get(1)
        .expect("There was a problem retrieving the filename from program args");

    let full_contained_assignments_count = parse_assignments(filename)
        .iter()
        .map(|parsed| (Assignment::create(&parsed.0), Assignment::create(&parsed.1)))
        .filter(|ap| ap.0.either_full_contained(&ap.1))
        .count();

    println!("{:?}", full_contained_assignments_count);
}

fn parse_assignments(filename: &str) -> Vec<(String, String)> {
    std::fs::read_to_string(filename)
        .expect(format!("There was a problem reading {}", filename).as_str())
        .lines()
        .map(|line| line.trim().split(',').collect())
        .map(|parts: Vec<&str>| (parts[0].to_owned(), parts[1].to_owned()))
        .collect()
}
