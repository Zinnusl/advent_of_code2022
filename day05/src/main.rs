struct Instruction(u32, u32, u32);

impl Instruction {
    fn from(input: &str) -> Self {
        let parts: Vec<&str> = input.split(' ').collect();
        let a = parts[1].parse().unwrap();
        let b = parts[3].parse().unwrap();
        let c = parts[5].parse().unwrap();

        Self(a, b, c)
    }
}

struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn new() -> Self {
        Self { stacks: Vec::new() }
    }

    fn input_line(&mut self, line: &str) {
        let mut stack = Vec::new();
        for ch in line.chars() {
            if ch == '[' || ch == ']' {
                continue;
            }

            stack.push(ch);
        }

        self.stacks.push(stack);
    }

    fn get_top_crates(&self) -> String {
        let mut output = String::new();

        for stack in &self.stacks {
            if stack.is_empty() {
                output.push('-');
            } else {
                output.push(stack[stack.len() - 1]);
            }
        }

        output
    }

    fn operate(&mut self, i: Instruction) {
        let (a, b, c) = (i.0, i.1, i.2);
        let crate_a = self.stacks[(b - 1) as usize].pop().unwrap();

        for _ in 0..a - 1 {
            let _crate = self.stacks[(b - 1) as usize].pop().unwrap();
        }

        self.stacks[(c - 1) as usize].push(crate_a);
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
            } else if line.contains('[') {
                stacks.input_line(line);
            }
        });

    for i in instructions {
        stacks.operate(i);
    }

    println!("Solution: {}", stacks.get_top_crates());

    // println!("Stacks: {:?}", stacks);
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
        assert_eq!(s.get_top_crates(), "AC");
        assert_eq!(s.stacks[0][0], 'B');
        assert_eq!(s.stacks[0][1], 'A');
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

        assert_eq!("A ", s.get_top_crates());
    }
}
