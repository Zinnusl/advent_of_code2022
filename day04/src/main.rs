type Section = i32;

#[derive(Debug, Copy, Clone)]
struct Elf(Section, Section);

impl Elf {
    fn overlaps(&self, other: &Elf) -> bool {
        self.0 <= other.1 && other.0 <= self.1 || other.0 <= self.1 && self.0 <= other.1
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pairs_that_contain_eachother = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut pairs = line.split(',');
            let elf0 = pairs.next().unwrap().split('-').map(|section| section.parse::<Section>().unwrap()).collect::<Vec<_>>();
            let elf1 = pairs.next().unwrap().split('-').map(|section| section.parse::<Section>().unwrap()).collect::<Vec<_>>();
            let elf0 = Elf(elf0[0], elf0[1]);
            let elf1 = Elf(elf1[0], elf1[1]);
            i32::from(elf0.overlaps(&elf1))
        })
        .sum::<i32>();

    println!("{:?}", pairs_that_contain_eachother);

    Ok(())
}
