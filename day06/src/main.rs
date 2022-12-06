use std::collections::VecDeque;
use std::collections::HashSet;

struct SignalCandidate {
    marker: VecDeque<char>
}

impl SignalCandidate {
    fn is_signal_marker(&self) -> bool {
        if self.marker.len() == 14 {
            let set = HashSet::new();
            let set = self.marker.iter().fold(set, |mut set, c| {
                set.insert(c);
                set
            });
            if set.len() == 14 {
                return true;
            }
        }

        false
    }

    fn next(&mut self, c: char) {
        self.marker.push_back(c);

        if self.marker.len() > 14 {
            self.marker.pop_front();
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Could not read input file");

    let mut sc = SignalCandidate { marker: VecDeque::new() };
    for (i, c) in input.chars().enumerate() {
        sc.next(c);

        if sc.is_signal_marker() {
            println!("Message marker at position {}", i - 13);
            println!("Solution {}", i + 1);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_signal_marker() {
        let a = SignalCandidate { marker: VecDeque::from(vec!['a', 'b', 'c', 'd']) };

        assert_eq!(a.is_signal_marker(), false);

        let mut a = SignalCandidate { marker: VecDeque::from(vec!['d', 'b', 'c', 'd', 'z', 'v', 'w', 'q', 'r', 'l', 'f', 'y', 'k']) };

        assert_eq!(a.is_signal_marker(), false);

        a.next('e');

        assert_eq!(a.is_signal_marker(), false);

        a.next('b');

        assert_eq!(a.is_signal_marker(), false);

        a.next('p');

        assert_eq!(a.is_signal_marker(), true);

        a.next('j');

        assert_eq!(a.is_signal_marker(), true);
    }
}
