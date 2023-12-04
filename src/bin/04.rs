use rayon::prelude::*;
use std::collections::HashSet;

advent_of_code::solution!(4);
#[derive(Debug)]
pub struct ScratchOff {
    matches: u32,
}
impl ScratchOff {
    fn new(matches: u32) -> Self {
        Self { matches }
    }
    fn get_score(&self) -> u32 {
        if self.matches == 0 {
            return 0;
        } else {
            return 2_u32.pow(self.matches - 1) as u32;
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .par_lines()
            .into_par_iter()
            .map(parse_line)
            .map(|c| c.get_score())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input
        .par_lines()
        .map(parse_line)
        .collect::<Vec<ScratchOff>>();
    let len = cards.len();
    let mut dupes: Vec<u32> = vec![1; len as usize];

    for (x, card) in cards.iter().enumerate() {
        if card.matches == 0 {
            continue;
        }
        let duped_times = dupes.get(x).copied().unwrap();
        let duped_cards = &mut dupes[x + 1..(x + card.matches as usize + 1)];
        for x in duped_cards {
            *x += duped_times
        }
    }
    Some(dupes.iter().sum())
}

pub fn parse_line(line: &str) -> ScratchOff {
    let temp = line
        .split(":")
        .last()
        .expect("there should be stuff after colon")
        .split('|')
        .map(|l| {
            l.split(" ")
                .filter(|n| *n != "")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>()
        })
        .collect::<Vec<HashSet<u32>>>();

    ScratchOff::new(
        temp.get(0)
            .unwrap()
            .intersection(&temp.get(1).unwrap())
            .count() as u32,
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
