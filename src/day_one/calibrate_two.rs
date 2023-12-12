pub fn calibrate_two(strs: Vec<&str>) -> usize {
    let possible_maches = HashMap::from([
        ('o', vec!["one"]),
        ('t', vec!["two", "three"]),
        ('s', vec!["six", "seven"]),
        ('f', vec!["four", "five"]),
        ('n', vec!["nine"]),
        ('e', vec!["eight"]),
    ]);

    let conversions = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("six", '6'),
        ("seven", '7'),
        ("four", '4'),
        ("five", '5'),
        ("nine", '9'),
        ("eight", '8'),
    ]);

    let mut sum: usize = 0;

    for line in strs {
        let mut first = 'n';
        let mut last = 'n';

        for (i, char) in line.chars().enumerate() {
            if char.is_numeric() {
                if first == 'n' {
                    first = char;
                    continue;
                }
                last = char;
                continue;
            }

            if possible_maches.contains_key(&char) {
                let numeric_matches = possible_maches
                    .get(&char)
                    .expect("There is no vector with that key: 64");

                if line.len() < i + 1 {
                    continue;
                }

                let final_posssible_idx = i + 5;

                let mut l_str = "".to_string();

                for y in i..final_posssible_idx {
                    if let Some(l_char) = line.chars().nth(y) {
                        l_str.push(l_char)
                    }
                }

                for p_match in numeric_matches {
                    if l_str.contains(p_match) {
                        if first == 'n' {
                            first = *conversions.get(p_match).expect("cannot convert: 97");
                            continue;
                        }
                        last = *conversions.get(p_match).expect("cannot convert: 100");
                        continue;
                    }
                }
            }
        }

        let mut local_str = first.to_string();
        if last != 'n' {
            local_str.push(last);
        } else {
            local_str.push(first);
        }
        let local_num = local_str.parse::<usize>().unwrap();
        sum += local_num;
    }

    sum
}

/*

    #[test]
    fn cal_two() {
        let input = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        let result = calibrate_two(input);
        println!("the result: {:?} ", result);
        assert_eq!(result, 281);
    }

*/
