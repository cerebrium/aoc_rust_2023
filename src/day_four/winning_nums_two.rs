use std::collections::HashMap;

fn winning_numbers_two(vec: Vec<&str>) -> usize {
    let mut overall_hashmap: HashMap<usize, usize> = HashMap::new();
    let mut available_cards = 0;
    let mut final_sum = 0;

    for (i, line) in vec.iter().enumerate() {
        available_cards += 1;
        overall_hashmap.insert(i, 0);
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
                let hash_value = overall_hashmap.get(&i);

                if let Some(val) = hash_value {
                    overall_hashmap.insert(i, val + 1);
                }
            }
            continue;
        }
    }

    let mut mult_hash: HashMap<usize, usize> = HashMap::new();

    for idx in 0..available_cards {
        mult_hash.insert(idx, 1);
    }

    for idx in 0..available_cards {
        let mut additional_amount: usize = 1;

        if let Some(overall_counts) = overall_hashmap.get(&idx) {
            if let Some(prev_mult_val) = mult_hash.get(&idx) {
                additional_amount = *prev_mult_val;
            }

            for i in (idx + 1)..(*overall_counts + 1 + idx) {
                if let Some(mult_val) = mult_hash.get_mut(&i) {
                    *mult_val += additional_amount;
                }
            }
        }

        final_sum += additional_amount;
    }

    final_sum
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

        let result = winning_numbers_two(test_data);
        println!("what is the result: {:?}", result);
        assert_eq!(result, 30);
    }
}
