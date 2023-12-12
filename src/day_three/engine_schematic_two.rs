#[derive(Debug)]
pub struct Num {
    value: String,
    start: usize,
    end: usize,
    row: usize,
}

#[derive(Debug)]
pub struct Char {
    idx: usize,
    row: usize,
    char: char,
}

// 497027 is too low

fn engine_schematic_two(schematic: Vec<&str>) -> usize {
    let mut sum: usize = 0;

    let mut found_nums: Vec<Num> = vec![];
    let mut found_chars: Vec<Char> = vec![];

    // Create the num structs vec and char structs vec
    for (outer_idx, line) in schematic.iter().enumerate() {
        let mut value = "".to_string();
        let mut start: usize = 8000;

        for (inner_idx, char) in line.trim().chars().enumerate() {
            let mut l_inner_idx = line.len() - 1;

            if inner_idx != line.len() {
                l_inner_idx = inner_idx;
            }

            if char.is_numeric() {
                if start == 8000 {
                    start = inner_idx + 1;
                }

                value.push(char);

                if inner_idx == line.len() - 1 {
                    found_nums.push(Num {
                        value,
                        start,
                        end: l_inner_idx,
                        row: outer_idx,
                    });

                    start = 8000;
                    value = "".to_string();
                }
                continue;
            }

            if start != 8000 {
                found_nums.push(Num {
                    value,
                    start,
                    end: l_inner_idx,
                    row: outer_idx,
                });

                start = 8000;
                value = "".to_string();
            }

            if char == '.' {
                continue;
            }

            found_chars.push(Char {
                idx: l_inner_idx + 1,
                row: outer_idx,
                char,
            })
        }
    }

    /*

    the indexing needs to not be done with usize
    be sighned int. Or just check for negatives
    in the loop below

    */

    for char in found_chars {
        // Only if it is an asteriks
        let mut matches = vec![];
        if char.char == '*' {
            for num in &found_nums {
                /*
                 *
                 * Rules:
                 *  1. If char row matches num row
                 *      a: char idx is 1 greater than num end
                 *      b: char idx is 1 less than num start
                 *  3: char row is within 1 of num row:
                 *      a. char idx falls within start - 1 through end +1
                 *
                 * Do:
                 *  1. convert num value to usize
                 *  2. add the number to the sum
                 *
                 */

                let mut min = 0;
                if char.idx >= 1 {
                    min = char.idx - 1
                }

                if char.row == num.row && (char.idx + 1 == num.start || min == num.end) {
                    matches.push(num);
                    break;
                }

                let mut min_row = 0;
                if char.row >= 1 {
                    min_row = char.row - 1
                }

                if char.row + 1 == num.row || min_row == num.row {
                    let mut local_start = 0;

                    if num.start >= 1 {
                        local_start = num.start - 1
                    }
                    for breadth in local_start..num.end + 2 {
                        if breadth == char.idx {
                            matches.push(num);
                            continue;
                        }
                    }
                }
            }
        }

        if matches.len() == 2 {
            let mut local_sum = 1;
            for num in &matches {
                local_sum *= num.value.parse::<usize>().unwrap();
            }

            sum += local_sum;
        }
    }

    sum
}

// 60308793 is too low
mod tests {
    use super::*;

    #[test]
    fn engine_schematic_two_test_one() {
        let test_data = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];

        let result = engine_schematic_two(test_data);
        println!("what is the result of test two: {:?}", result);
        assert_eq!(result, 467835);
    }
}
