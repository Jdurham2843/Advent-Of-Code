fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Please supply a rucksacks input file.");
        std::process::exit(1);
    }

    let filename = args.get(1).unwrap();

    let all_items: Vec<char> = String::from_utf8(
        (b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap().chars().collect();

    let total: u16 = parse_rucksack_groups(&filename)
        .iter()
        .map(|group| find_common_item(group))
        .map(|item| 53 - (52 - all_items.iter().position(|&i| i== item).unwrap()) as u16)
        .sum();

    println!("{:?}", total);
}

fn parse_rucksack_groups(filename: &str) -> Vec<(String, String, String)> {
    std::fs::read_to_string(filename)
        .expect(format!("There was a problem parsing {}.", filename).as_str())
        .lines()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|chunk| (chunk[0].to_owned(), chunk[1].to_owned(), chunk[2].to_owned()))
        .collect()
}

fn find_common_item(rucksack: &(String, String, String)) -> char {
    let first_rucksack: Vec<char>  = rucksack.0.chars().collect();
    let second_rucksack: Vec<char> = rucksack.1.chars().collect();
    let third_rucksack: Vec<char>  = rucksack.2.chars().collect();

    first_rucksack.iter()
        .filter(|item| second_rucksack.contains(&item))
        .filter(|item| third_rucksack.contains(&item))
        .take(1)
        .next()
        .expect("Failure: there should have been atleast one character in common.")
        .to_owned()
}
