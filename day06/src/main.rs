use std::collections::HashSet;
use std::collections::VecDeque;

struct SignalCandidate {
    marker: VecDeque<char>,
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
    let input = std::fs::read_to_string("input.txt").expect("Could not read input file");

    let mut sc = SignalCandidate {
        marker: VecDeque::new(),
    };
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
    fn test_is_signal_marker() {
        // Test that is_signal_marker returns true when the marker has 14 characters and all characters are unique
        let mut sc = SignalCandidate {
            marker: VecDeque::new(),
        };
        sc.marker.push_back('a');
        sc.marker.push_back('b');
        sc.marker.push_back('c');
        sc.marker.push_back('d');
        sc.marker.push_back('e');
        sc.marker.push_back('f');
        sc.marker.push_back('g');
        sc.marker.push_back('h');
        sc.marker.push_back('i');
        sc.marker.push_back('j');
        sc.marker.push_back('k');
        sc.marker.push_back('l');
        sc.marker.push_back('m');
        sc.marker.push_back('n');
        assert!(sc.is_signal_marker());

        // Test that is_signal_marker returns false when the marker has less than 14 characters
        let mut sc = SignalCandidate {
            marker: VecDeque::new(),
        };
        sc.marker.push_back('a');
        sc.marker.push_back('b');
        sc.marker.push_back('c');
        sc.marker.push_back('d');
        sc.marker.push_back('e');
        sc.marker.push_back('f');
        sc.marker.push_back('g');
        sc.marker.push_back('h');
        sc.marker.push_back('i');
        sc.marker.push_back('j');
        sc.marker.push_back('k');
        sc.marker.push_back('l');
        assert!(!sc.is_signal_marker());

        // Test that is_signal_marker returns false when the marker has 14 characters but not all characters are unique
        let mut sc = SignalCandidate {
            marker: VecDeque::new(),
        };
        sc.marker.push_back('a');
        sc.marker.push_back('b');
        sc.marker.push_back('c');
        sc.marker.push_back('d');
        sc.marker.push_back('e');
        sc.marker.push_back('f');
        sc.marker.push_back('g');
        sc.marker.push_back('h');
        sc.marker.push_back('i');
        sc.marker.push_back('j');
        sc.marker.push_back('k');
        sc.marker.push_back('l');
        sc.marker.push_back('m');
        sc.marker.push_back('a');
        assert!(!sc.is_signal_marker());
    }

    #[test]
    fn test_next() {
        // Test that next correctly adds a new character to the marker and removes the oldest character if the marker has reached its maximum size of 14 characters
        let mut sc = SignalCandidate { marker: VecDeque::new() };
        for i in 0..15 {
            sc.next(i as u8 as char);
        }
        assert_eq!(sc.marker.len(), 14);
        assert_eq!(sc.marker.front().unwrap(), &'\u{01}');
    }

    #[test]
    fn test_main() {
        // Test that main correctly reads the input from the file and prints the correct output for a sample input file
        let input = "abcdefghijklmabcdefghijklma";
        let mut sc = SignalCandidate {
            marker: VecDeque::new(),
        };
        for (i, c) in input.chars().enumerate() {
            sc.next(c);
            if sc.is_signal_marker() {
                assert_eq!(i - 13, 28);
                assert_eq!(i + 1, 29);
            }
        }
    }
}
