fn is_symbol(c: &char) -> bool {
    if c.is_digit(10) {
        return false;
    }
    if c == &'.' {
        return false;
    }
    return true;
}

fn solve_part1(inp: String) -> usize {
    let graph: Vec<Vec<char>> = inp.split('\n').map(|x| x.chars().collect()).collect();

    for (i, row) in graph.iter().enumerate() {
        let mut accept_row = false;
        for (j, val) in row.iter().enumerate() {
            if is_symbol(val) {
                accept_row = true;
            }
        }
    }

    return 0;
}

fn main() {
    println!("day3");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let inp = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .to_string();

        assert_eq!(solve_part1(inp), 4361);
    }
}
