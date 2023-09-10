pub fn part_one(input: &str) -> Option<u32> {

    // Group lines by blank lines
    let res = input.trim()
         .lines()
         .fold(vec![0], |mut acc, line| {
            if line.is_empty() {
                acc.push(0);
            } else {
                let last = acc.last_mut().unwrap();
                *last = *last + line.parse::<u32>().unwrap();
            }
            acc
         });

    Some(res.iter().max().unwrap().clone())
}

pub fn part_two(input: &str) -> Option<u32> {
    // Group lines by blank lines
    let mut res = input.trim()
         .lines()
         .fold(vec![0], |mut acc, line| {
            if line.is_empty() {
                acc.push(0);
            } else {
                let last = acc.last_mut().unwrap();
                *last = *last + line.parse::<u32>().unwrap();
            }
            acc
         });
    
    res.sort_by_key(|&x| std::cmp::Reverse(x));

    Some(res.iter().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
