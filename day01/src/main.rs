struct Elf {
    calories: u32,
}

fn main() {
    let mut elfs = vec![];
    let mut calories = 0;
    for line in std::fs::read_to_string("input.txt") 
        .expect("Error reading file")
        .lines() {
            match line {
                "" => {
                    elfs.push(Elf { calories });
                    calories = 0;
                }
                _ => {
                    calories += line.parse::<u32>().unwrap();
                }
            };
    }

    elfs.push(Elf { calories });

    elfs.sort_by(|a, b| b.calories.cmp(&a.calories));
    println!("Max: {:?}", elfs[0].calories + elfs[1].calories + elfs[2].calories);
}
