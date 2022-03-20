use std::fs;

#[derive(Debug)]
enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("Please supply a position playbook file.");
        std::process::exit(1);
    }

    let filename = args.get(0).unwrap();

    let instructions = extract_instructions(filename);

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Forward(n) => { 
                x += n;
                y += aim * n;
            },
            Instruction::Down(n) => { 
                aim += n;
            },
            Instruction::Up(n) => {
                aim -= n
            },
        }
    }

    println!("Final coordinates are: x = {}, y = {}", x, y);
    println!("Multiplied together, they are: {}", x * y);
}

fn extract_instructions(filename: &str) -> Vec<Instruction> {
    let file_contents = extract_contents_from_file(filename);

    let mut instructions = Vec::new();
    for item in file_contents {
        if let Some(instruction) = parse_instruction(item) {
            instructions.push(instruction);
        }
    }

    instructions
}

fn extract_contents_from_file(filename: &str) -> Vec<String> {
    let file_contents = fs::read_to_string(filename)
        .expect(format!("There was a problem reading {}", filename).as_str());
    
    file_contents.split('\n')
        .map(|s| s.trim().to_string())
        .collect()
}

fn parse_instruction(instruction_input: String) -> Option<Instruction> {
    let instruction_parts: Vec<String> = instruction_input
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    if instruction_parts.len() != 2 {
        return None;
    }

    let command = instruction_parts.get(0)?;
    let count: u32 = match instruction_parts.get(1)?.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Faild to parse {} {}", command, instruction_parts.get(1)?);
            return None;
        },
    };

    match command.as_str() {
        "forward" => {
            Some(Instruction::Forward(count))
        },
        "down" => {
            Some(Instruction::Down(count))
        },
        "up" => {
            Some(Instruction::Up(count))
        },
        _ => {
            eprintln!("Failed to parse {} {}", command, count);
            None
        }
    }
}