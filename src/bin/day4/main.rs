use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn card_value(line: &str) -> usize {
    let (_, cards) = line.split_once(":").unwrap();
    let (winning, yours) = cards.split_once("|").unwrap();
    let winning: Vec<usize> = winning
        .split(" ")
        .filter_map(|x| x.trim().parse().ok())
        .collect();
    let yours: Vec<usize> = yours
        .split(" ")
        .filter_map(|x| x.trim().parse().ok())
        .collect();

    let mut points = 0;
    for x in &winning {
        for y in &yours {
            if x == y {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
                continue;
            }
        }
    }

    return points;
}

fn num_cards(line: &str, cards: &mut HashMap<usize, usize>) {
    let (card_name, nums) = line.split_once(":").unwrap();
    let (_, card_num) = card_name.split_once(" ").unwrap();
    let card_num = card_num.trim().parse::<usize>().unwrap();
    let (winning, yours) = nums.split_once("|").unwrap();

    let winning: Vec<usize> = winning
        .split(" ")
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    let yours: Vec<usize> = yours
        .split(" ")
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    let mut win_count = 0;
    for x in &winning {
        for y in &yours {
            if x == y {
                win_count += 1;
                continue;
            }
        }
    }

    let mut other_cards = Vec::new();
    for i in 0..win_count {
        other_cards.push(i + 1 + card_num);
    }

    cards.insert(card_num, cards.get(&card_num).unwrap_or(&0) + 1);
    if let Some(c) = cards.get(&card_num) {
        for _ in 0..*c {
            for t in &other_cards {
                cards.insert(*t, cards.get(t).unwrap_or(&0) + 1);
            }
        }
    }
}

fn main() {
    let f = File::open("./src/bin/day4/input.txt").expect("file not found");
    let reader = BufReader::new(f);
    let mut total = 0;
    let mut cards: HashMap<usize, usize> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        total += card_value(&line);
        num_cards(&line, &mut cards);
    }

    let mut part2 = 0;
    for (_, v) in cards {
        part2 += v;
    }
    println!("part 1 = {}", total);
    println!("part 2 = {}", part2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let mut total = 0;
        let f = File::open("./src/bin/day4/example1.txt").expect("file not found");
        let reader = BufReader::new(f);
        for line in reader.lines() {
            total += card_value(&line.unwrap());
        }

        assert_eq!(total, 13);
    }

    #[test]
    fn test_part_two() {
        let f = File::open("./src/bin/day4/example1.txt").expect("file not found");
        let reader = BufReader::new(f);
        let mut cards: HashMap<usize, usize> = HashMap::new();
        for line in reader.lines() {
            num_cards(&line.unwrap(), &mut cards)
        }
        println!("{:?}", cards);

        let mut part2 = 0;
        for (_, v) in cards {
            part2 += v;
        }
        assert_eq!(part2, 30);
    }
}
