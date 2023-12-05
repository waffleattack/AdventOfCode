use itertools::Itertools;
advent_of_code::solution!(5);

struct Mappings {
    maps: Vec<Vec<Rule>>,
}
struct Rule {
    destination: i64,
    start: i64,
    range: i64,
}
struct SeedRange {
    start: i64,
    end: i64,
}
impl Rule {
    fn check_range(&self, num: i64) -> bool {
        self.start < num && num < self.start + self.range
    }
    fn get_offset(&self) -> i64 {
        self.destination - self.start
    }
}
impl Mappings {
    fn run_map(&self, seed: i64) -> i64 {
        let mut location = seed;
        self.maps.iter().for_each(|m| {
            location += m
                .iter()
                .find(|r| r.check_range(location))
                .map_or(0, |r| r.get_offset());
        });

        location
    }
}
pub fn part_one(input: &str) -> Option<i64> {
    let (seeds_unstripped, maps_str) = input.split_once("\n\n").unwrap();
    let seeds = seeds_unstripped.strip_prefix("seeds: ").unwrap();
    let seeds = seeds.split_whitespace().map(|s| s.parse::<i64>().unwrap());
    let rules = maps_str
        .split("\n\n")
        .map(|m| {
            m.lines()
                .skip(1)
                .map(|range| {
                    let mut numbers = range.split(" ").filter_map(|n| n.parse::<i64>().ok());
                    Rule {
                        destination: numbers.next().unwrap(),
                        start: numbers.next().unwrap(),
                        range: numbers.next().unwrap(),
                    }
                })
                .collect()
        })
        .collect();
    let map = Mappings { maps: rules };
    seeds.map(|n| map.run_map(n)).min()
}

pub fn part_two(input: &str) -> Option<i64> {
    let (seeds_unstripped, maps_str) = input.split_once("\n\n").unwrap();
    let binding = seeds_unstripped
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .chunks(2);
    let seeds = binding.into_iter().map(|mut c|{
        let start = c.next().unwrap();
        SeedRange{
            start,
            end: start + c.next().unwrap()
        }
    });
    let maps = maps_str
        .split("\n\n")
        .map(|m| {
            m.lines()
                .skip(1)
                .map(|range| {
                    let mut numbers = range.split(" ").filter_map(|n| n.parse::<i64>().ok());
                    Rule {
                        destination: numbers.next().unwrap(),
                        start: numbers.next().unwrap(),
                        range: numbers.next().unwrap(),
                    }
                }).collect_vec()

        }).collect_vec();

    let mut curr_seed_ranges = seeds.collect_vec();
    for map in &maps{
        let mut transformed_seeds:Vec<SeedRange> = Vec::new();
        while let Some(seed_range)=curr_seed_ranges.pop(){
            let mut found = false;
            for rule in map {
                let rule_end = rule.start + rule.range;

                let to_right = seed_range.start >= rule.start && seed_range.start < rule_end;
                let to_left = seed_range.end <= rule.start+rule.range && seed_range.end > rule.start;
                let offset = rule.get_offset();

                if to_right && to_left{
                    found = true;
                    transformed_seeds.push(SeedRange{
                        start: seed_range.start + offset,
                        end: seed_range.end + offset,
                    });
                    break;
                }
                //
                if to_right {
                    found = true;

                    transformed_seeds.push(SeedRange{
                        start: seed_range.start + offset,
                        end: rule_end+offset
                    });
                    curr_seed_ranges.push(SeedRange{
                        start: rule_end,
                        end: seed_range.end
                    });
                    break;
                }
                if to_left{
                    found = true;

                    transformed_seeds.push(SeedRange{
                        start: rule.start+offset,
                        end: seed_range.end+offset
                    });
                    curr_seed_ranges.push(SeedRange{
                        start: seed_range.start,
                        end: rule.start-1
                    });
                    break;
                }

            }
            if !found {
                transformed_seeds.push(seed_range)
            }


        }
        curr_seed_ranges = transformed_seeds;
    };


    curr_seed_ranges.iter().map(|range|range.start ).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
