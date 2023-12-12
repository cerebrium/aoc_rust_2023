use std::collections::HashMap;

fn winning_numbers(vec: Vec<&str>) -> usize {
    let mut total_amount = 0;

    for line in vec {
        let mut local_sum = 0;
        let matches: Vec<&str> = line.trim().split("|").collect();

        let tuple_vec_matching_nums: Vec<&str> = matches[0].split(" ").collect();
        let mut winning_nums: HashMap<&str, bool> = HashMap::new();

        tuple_vec_matching_nums.iter().for_each(|l_match| {
            if !l_match.is_empty() {
                winning_nums.insert(l_match.trim(), true);
            }
        });

        for potential_match in matches[1].split(" ") {
            if winning_nums.contains_key(potential_match) {
                if local_sum == 0 {
                    local_sum += 1;
                } else {
                    local_sum *= 2;
                }
            };
        }

        total_amount += local_sum
    }

    total_amount
}

mod tests {
    use super::*;

    #[test]

    fn test_winning_numbers() {
        let test_data = vec![
            " 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            " 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            " 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            " 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            " 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        let result = winning_numbers(test_data);
        assert_eq!(result, 13);
    }
}
