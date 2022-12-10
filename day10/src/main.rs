enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn cycles(&self) -> i32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_x) => 2,
        }
    }
}

fn test(cycle: i32, reg: i32, crt: &mut String) {
    let mut rem = cycle % 40;
    if rem == 0 {
        rem = 40;
    }
    {
        match ((rem-2)..=(rem)).contains(&reg) {
            true => crt.push_str("#"),
            false => crt.push_str("."),
        }
    }
    if rem == 40 {
        crt.push_str("\n");
    }
}

fn compute(input: String) -> String {
    let re = regex::Regex::new(r"^(noop|addx) ?(-?\d+)?$").unwrap();
    let mut input = input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            match &caps[1] {
                "noop" => Instruction::Noop,
                "addx" => Instruction::Addx(caps[2].parse().unwrap()),
                _ => unreachable!(),
            }
        });

    let mut crt: String = "".to_string();
    let mut reg = 1;
    let mut cycle = 1;

    while let Some(i) = input.next() {
        let c = 0..i.cycles();
        match i {
            Instruction::Noop => {
                for _ in c {
                    test(cycle, reg, &mut crt);
                    cycle += 1;
                }
                ()
            },
            Instruction::Addx(x) => {
                for _ in c {
                    test(cycle, reg, &mut crt);
                    cycle += 1;
                }
                reg += x
            }
        };
    }

    crt
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let crt = compute(input);

    println!("{}", crt);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_test() {
        let input = "noop
noop
noop
noop";

        let expected = "###.";

        assert_eq!(compute(input.to_string()), expected);
    }

    #[test]
    fn simple_test() {
        let input = "addx 15
addx -11
addx 6
addx -3";

        let expected = "##..##..";

        assert_eq!(compute(input.to_string()), expected);
    }

    #[test]
    fn hard_test() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

        assert_eq!(compute(input.to_string()), expected);
    }
}
