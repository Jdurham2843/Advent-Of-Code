use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() != 1 {
        println!("I require a filename!");
        std::process::exit(1);
    }

    let filename = args.get(0).unwrap();
    let depth_readings = extract_readings(filename);
    let depth_window_sums = calculate_window_sums(depth_readings);

    println!("{:?}", depth_window_sums);

    let mut increase_count = 0;
    let mut previous = 0;

    for (pos, sum) in depth_window_sums.iter().enumerate() {
        if pos == 0 {
            previous = *sum;
            continue;
        }

        if *sum > previous {
            increase_count += 1;
        }

        previous = *sum;
    }

    println!("The depth window sum increased {} times.", increase_count);
}

fn extract_readings(filename: &str) -> Vec<u16> {
    let file_contents = fs::read_to_string(filename)
        .expect(format!("There was a problem opening {}", filename).as_str());

    file_contents.split('\n')
        .map(|s| s.trim().to_string().parse().unwrap())
        .collect()
}

fn calculate_window_sums(depth_readings: Vec<u16>) -> Vec<u16> {
    let mut window_sums = Vec::new();
    
    for index in (0..depth_readings.len()) {
        if index < 2 {
            continue;
        }

        let sum = depth_readings.get(index).unwrap() + 
                  depth_readings.get(index - 1).unwrap() +
                  depth_readings.get(index - 2).unwrap();
        window_sums.push(sum);
    }

    window_sums
}
