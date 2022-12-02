use std::fs;

#[derive(Debug)]
struct Elf {
    items: Vec<u32>
}

impl Elf {
    fn total_calories(&self) -> u32 {
        self.items.iter().sum()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Please supply a file containing calories per item per elf.");
        std::process::exit(1);
    }

    let filename = args.get(1).unwrap();
    let elves = parse_elves(filename);

    let mut max_elf: Option<Elf> = None;
    for elf in elves {
        match max_elf {
            Some(prev_max_elf) => {
                max_elf = if elf.total_calories() > prev_max_elf.total_calories() { Some(elf) } else { Some(prev_max_elf) }; 
            },
            None => { max_elf = Some(elf); },
        }
    }

    println!("Max calories: {}", max_elf.unwrap().total_calories());
}

fn parse_elves(filename: &str) -> Vec<Elf> {
    let file_contents = fs::read_to_string(filename)
        .expect(format!("There was a problem reading {}", filename).as_str());

    let mut elves: Vec<Elf> = Vec::new();
    
    let mut calories: Vec<u32> = Vec::new();
    for line in file_contents.lines() {
        if line.trim().is_empty() {
            elves.push(Elf { items: calories });
            calories = Vec::new();
        } else {
            calories.push(line.parse::<u32>().unwrap());
        }
    }

    if !calories.is_empty() {
        elves.push(Elf { items: calories });
    }

    
    elves
}
