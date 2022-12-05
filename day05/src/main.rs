use std::collections::VecDeque;
use regex::Regex;

type Stack = VecDeque<char>;

struct Instruction(i32, i32, i32);

impl From<&str> for Instruction {
    fn from(s: &str) -> Instruction {
        let re = Regex::new(r"\d+").unwrap();
        let m = re.find_iter(s).map(|k| k.as_str().parse::<i32>().unwrap()).collect::<Vec<_>>();
        Instruction ( m[0], m[1], m[2], )
    }
}

impl Instruction {
    pub fn operate(self, stacks: &mut Stacks) {
        let (amount, from, to) = (self.0, self.1, self.2);
        let (from, to) = (from as usize - 1, to as usize - 1);

        let mut crane = Stack::new();
        for _ in 1..=amount {
            crane.push_front(stacks.stacks[from].pop_back().unwrap());
        }
        for _ in 1..=amount {
            let c = crane.pop_front().unwrap();
            stacks.stacks[to].push_back(c);
        }
    }
}

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Stack>
}

impl Stacks {
    pub fn new() -> Stacks {
        Stacks { stacks: vec![] }
    }

    pub fn input_line(&mut self, line: &str) {
        let re = Regex::new(r"    |\[[A-Z]\]").unwrap();
        // for (i, m) in line.matches(re).enumerate() {
        for (i, m) in re.find_iter(line).enumerate() {
            while (i + 1) > self.stacks.len() {
                self.stacks.push(Stack::new());
            }
            let m = m.as_str();
            if m != "    " {
                self.stacks[i].push_front(
                    m.matches(char::is_alphabetic).next().unwrap().chars().next().unwrap()
                );
            }
        }
    }

    pub fn operate(&mut self, i: Instruction) {
        i.operate(self);
    }

    pub fn get_top_crates(&self) -> String {
        self.stacks.iter().map(|s| s.back().unwrap_or(&'-').to_string()).collect()
    }
}

fn main() {
    let mut stacks = Stacks::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    std::fs::read_to_string("input.txt")
        .expect("Could not read input file")
        .lines()
        .for_each(|line| {
            if line.starts_with("move") {
                instructions.push(Instruction::from(line));
            }
            else if line.contains('[') {
                stacks.input_line(line);
            }
        });

    for i in instructions {
        stacks.operate(i);
    }

    println!("Solution: {}", stacks.get_top_crates());

    println!("Stacks: {:?}", stacks);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instruction() {
        let i = Instruction::from("move 1 from 2 to 3");
        assert_eq!(i.0, 1);
        assert_eq!(i.1, 2);
        assert_eq!(i.2, 3);
    }

    #[test]
    fn parse_stack() {
        let mut s = Stacks::new();
        s.input_line("[A]");
        s.input_line("[B] [C]");
        assert_eq!(s.stacks[0][1], 'A');
        assert_eq!(s.stacks[0][0], 'B');
        assert_eq!(s.stacks[1][0], 'C');
    }

    #[test]
    fn get_top_crates() {
        let mut s = Stacks::new();
        s.input_line("[A]");
        s.input_line("[B] [C]");
        assert_eq!("AC", s.get_top_crates());
    }

    #[test]
    fn operate() {
        let mut s = Stacks::new();
        s.input_line("[A]");
        s.input_line("[B] [C]");

        let i = Instruction::from("move 1 from 1 to 2");

        s.operate(i);

        let i = Instruction::from("move 2 from 2 to 1");

        s.operate(i);

        assert_eq!("A-", s.get_top_crates());
    }
}
