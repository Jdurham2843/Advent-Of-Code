fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Please supply a rucksacks input file.");
        std::process::exit(1);
    }

    let filename = args.get(1).unwrap();

    let all_items: Vec<char> = String::from_utf8(
        (b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap().chars().collect();

    let total: u16 = parse_rucksacks(&filename)
        .iter()
        .map(|rucksack| find_common_item(rucksack))
        .map(|item| 53 - (52 - all_items.iter().position(|&i| i== item).unwrap()) as u16)
        .sum();

    println!("{:?}", total);
}

fn parse_rucksacks(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .expect(format!("There was a problem parsing {}.", filename).as_str())
        .lines()
        .map(|s| s.to_owned())
        .collect()
}

fn find_common_item(rucksack: &str) -> char {
    let midpoint = rucksack.len() / 2;

    let first_compartment: Vec<char>  = rucksack[..midpoint].chars().collect();
    let second_compartment: Vec<char> = rucksack[midpoint..].chars().collect();

    first_compartment.iter()
        .filter(|item| second_compartment.contains(&item))
        .take(1)
        .next()
        .expect("Failure: there should have been atleast one character in common.")
        .to_owned()
}
