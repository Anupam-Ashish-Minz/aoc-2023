use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq)]
struct Balls {
    game_id: usize,
    blue: usize,
    red: usize,
    green: usize,
}

fn parse_balls_max(s: &str) -> Balls {
    let (gs, colors) = s.split_once(":").unwrap();
    let (_, gid) = gs.split_once(" ").unwrap();
    let game_id = gid.parse::<usize>().unwrap();
    let mut balls = Balls {
        game_id,
        blue: 0,
        red: 0,
        green: 0,
    };
    for c in colors.split(|x| x == ',' || x == ';') {
        let (count, colr) = c.trim().split_once(" ").unwrap();
        let count = count.parse::<usize>().unwrap();
        if colr == "blue" && count > balls.blue {
            balls.blue = count
        } else if colr == "red" && count > balls.red {
            balls.red = count;
        } else if colr == "green" && count > balls.green {
            balls.green = count;
        }
    }
    return balls;
}

fn quality_games(b: &Balls, red: usize, green: usize, blue: usize) -> bool {
    return b.red <= red && b.green <= green && b.blue <= blue;
}

fn main() {
    let f = File::open("src/bin/day2/input.txt").expect("file not found");
    let reader = BufReader::new(f);
    let mut ids = 0;
    let mut power = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let b = parse_balls_max(&line);
        // println!("{} {} {} {}", b.game_id, b.red, b.green, b.blue);
        if quality_games(&b, 12, 13, 14) {
            ids += b.game_id;
        }

        let b2 = parse_balls_max(&line);
        power += b2.red * b2.green * b2.blue;
    }

    println!("part one ans = {}", ids);
    println!("part two ans = {}", power);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ball_parsing() {
        let game1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game2 = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let game3 = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game4 = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let game5 = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(
            parse_balls_max(game1),
            Balls {
                game_id: 1,
                blue: 6,
                red: 4,
                green: 2,
            }
        );
        assert_eq!(
            parse_balls_max(game2),
            Balls {
                game_id: 2,
                blue: 4,
                red: 1,
                green: 3,
            }
        );
        assert_eq!(
            parse_balls_max(game3),
            Balls {
                game_id: 3,
                blue: 6,
                red: 20,
                green: 13,
            }
        );
        assert_eq!(
            parse_balls_max(game4),
            Balls {
                game_id: 4,
                blue: 15,
                red: 14,
                green: 3,
            }
        );
        assert_eq!(
            parse_balls_max(game5),
            Balls {
                game_id: 5,
                blue: 2,
                red: 6,
                green: 3,
            }
        );
    }

    #[test]
    fn test_qualifier_games() {
        let game1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game2 = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let game3 = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game4 = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let game5 = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert!(quality_games(&parse_balls_max(game1), 12, 13, 14));
        assert!(quality_games(&parse_balls_max(game2), 12, 13, 14));
        assert!(!quality_games(&parse_balls_max(game3), 12, 13, 14));
        assert!(!quality_games(&parse_balls_max(game4), 12, 13, 14));
        assert!(quality_games(&parse_balls_max(game5), 12, 13, 14));
    }

    #[test]
    fn test_part_one() {
        let mut ids = 0;
        let inputs = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        for line in inputs {
            let b = parse_balls_max(&line);
            if quality_games(&b, 12, 13, 14) {
                ids += b.game_id;
            }
        }

        assert_eq!(ids, 8);
    }

    #[test]
    fn test_part_two() {
        let inputs = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let mut power = 0;
        for line in inputs {
            let b2 = parse_balls_max(&line);
            println!("{} {} {}", b2.red, b2.blue, b2.green);
            power += b2.red * b2.green * b2.blue;
        }

        assert_eq!(power, 2286);
    }
}
