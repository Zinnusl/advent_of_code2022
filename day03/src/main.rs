#![feature(iter_array_chunks)]

type Compartment = String;
type CommonElements = String;

#[derive(Debug, Clone)]
struct Rucksack(Compartment, Compartment);

impl Rucksack {
    fn common_elements(&self) -> CommonElements {
        let mut temp = self.1.clone();
        self.0.matches(|c| {
            temp.find(c).map(|i| temp.remove(i)).is_some()
        }).collect()
    }
    fn both(&self) -> String {
        self.0.to_owned() + &self.1
    }
}

trait SumValuesOfCompartment {
    fn sum_values(&self) -> i32;
}
impl SumValuesOfCompartment for String {
    fn sum_values(&self) -> i32 {
        let mut numbers = self.chars().map(|c| match c {
            'a'..='z' => c as i32 - 96,
            'A'..='Z' => c as i32 - 64 + 26,
            _ => panic!("Invalid character"),
        }).collect::<Vec<i32>>();

        numbers.sort();
        numbers.dedup();
        numbers.iter().sum()
    }
}

fn find_badge(string0: &str, string1: &str, string2: &str) -> i32 {
    for x in string0.chars() {
        if string0.matches(x).count() >= 1 && string1.matches(x).count() >= 1 && string2.matches(x).count() >= 1 {
            return match x {
                'a'..='z' => x as i32 - 96,
                'A'..='Z' => x as i32 - 64 + 26,
                _ => 0
            };
        }
    }

    0
}

impl SumValuesOfCompartment for BadgeGroup {
    fn sum_values(&self) -> i32 {
        let string0 = self.0.both();
        let string1 = self.1.both();
        let string2 = self.2.both();

        find_badge(&string0, &string1, &string2)
    }
}

#[derive(Debug, Clone)]
struct BadgeGroup(Rucksack, Rucksack, Rucksack);

fn main() {
    let comp_values_sum = std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .lines()
        .map(|line| {
            let line_matches = line.split_at(line.len() / 2);
            Rucksack(line_matches.0.to_string(), line_matches.1.to_string())
        })
        .array_chunks::<3>()
        .map(|rucksack| BadgeGroup(rucksack[0].clone(), rucksack[1].clone(), rucksack[2].clone()))
        // .take(10)
        // .map(|common_elements| (common_elements.clone(), common_elements.sum_values()))
        // .collect::<Vec<_>>();
        .map(|common_elements| common_elements.sum_values())
        .sum::<i32>();

    println!("Part 1: {:?}", comp_values_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_elements() {
        let rucksack = Rucksack("abc".to_string(), "cba".to_string());
        assert_eq!(rucksack.common_elements(), "abc");
    }

    #[test]
    fn test_badgegroup_sum_values() {
        assert_eq!(BadgeGroup(
            Rucksack("abc".to_string(), "zHG".to_string()),
            Rucksack("BE".to_string(), "zZXG".to_string()),
            Rucksack("DE".to_string(), "VXRz".to_string()),
        ).sum_values(), 26);
    }

    #[test]
    fn testerino() {
        assert_eq!('a' as i32, 97);
        assert_eq!('A' as i32, 65);
    }
}
