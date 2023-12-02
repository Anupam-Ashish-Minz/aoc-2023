use std::fs::File;
use std::io::{BufRead, BufReader};

fn solveline(s: &str) -> i32 {
    let mut start: i32 = -1;
    let mut end: i32 = 0;
    s.chars().for_each(|x| {
        if x.is_digit(10) {
            if start == -1 {
                start = x.to_digit(10).unwrap().try_into().unwrap();
            }
            end = x.to_digit(10).unwrap().try_into().unwrap();
        }
    });
    return start * 10 + end;
}

fn solveline2(s: &str) -> i32 {
    let mut start: i32 = -1;
    let mut end: i32 = 0;
    let digs = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    s.chars().enumerate().for_each(|(i, x)| {
        if x.is_digit(10) {
            if start == -1 {
                start = x.to_digit(10).unwrap().try_into().unwrap();
            }
            end = x.to_digit(10).unwrap().try_into().unwrap();
        } else {
            for (j, y) in digs.iter().enumerate() {
                if i + y.len() <= s.len() && &s[i..i + y.len()] == *y {
                    if start == -1 {
                        start = j as i32 + 1;
                    }
                    end = j as i32 + 1;
                }
            }
        }
    });
    return start * 10 + end;
}

fn main() {
    let f = File::open("src/bin/day1/input.txt").expect("input file not found");
    let reader = BufReader::new(f);
    let mut total1 = 0;
    let mut total2 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        total1 += solveline(&line);
        total2 += solveline2(&line);
    }
    println!("part one ans = {}", total1);
    println!("part two ans = {}", total2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let line1 = "1abc2";
        let line2 = "pqr3stu8vwx";
        let line3 = "a1b2c3d4e5f";
        let line4 = "treb7uchet";

        assert_eq!(solveline(line1), 12);
        assert_eq!(solveline(line2), 38);
        assert_eq!(solveline(line3), 15);
        assert_eq!(solveline(line4), 77);
    }

    #[test]
    fn test_part_two() {
        let line1 = "two1nine";
        let line2 = "eightwothree";
        let line3 = "abcone2threexyz";
        let line4 = "xtwone3four";
        let line5 = "4nineeightseven2";
        let line6 = "zoneight234";
        let line7 = "7pqrstsixteen";

        assert_eq!(solveline2(line1), 29);
        assert_eq!(solveline2(line2), 83);
        assert_eq!(solveline2(line3), 13);
        assert_eq!(solveline2(line4), 24);
        assert_eq!(solveline2(line5), 42);
        assert_eq!(solveline2(line6), 14);
        assert_eq!(solveline2(line7), 76);
    }
}
