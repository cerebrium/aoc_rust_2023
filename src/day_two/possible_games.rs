fn possible_games(combos: Vec<&str>, allowed_amounts: [i32; 3]) -> usize {
    use std::collections::HashMap;

    let mut sum = 0;
    let mut maxes = HashMap::from([('b', 0), ('r', 0), ('g', 0)]);

    let m_b = allowed_amounts[2];
    let m_r = allowed_amounts[0];
    let m_g = allowed_amounts[1];

    for (i, line) in combos.iter().enumerate() {
        let bags = line.trim().split(";");
        bags.for_each(|split| {
            let dice = split.split(",");

            dice.for_each(|combo| {
                let mut first_letter_of_color = 'z';
                let mut a_amount = "".to_string();

                for char in combo.chars() {
                    if char == ' ' {
                        continue;
                    }

                    if char.is_numeric() {
                        a_amount.push(char)
                    } else {
                        first_letter_of_color = char;
                        break;
                    }
                }

                let amount = a_amount.parse::<i32>().unwrap();

                let current_color_sum = maxes
                    .get(&first_letter_of_color)
                    .expect("could not find the hashmap value");

                if current_color_sum < &amount {
                    maxes.insert(first_letter_of_color, amount);
                }
            });
        });

        let b = maxes.get(&'b').unwrap();
        let r = maxes.get(&'r').unwrap();
        let g = maxes.get(&'g').unwrap();

        if b <= &m_b && r <= &m_r && g <= &m_g {
            sum += i + 1;
        }

        maxes = HashMap::from([('b', 0), ('r', 0), ('g', 0)]);
    }

    sum
}

mod test {

    use super::*;

    #[test]
    fn cal_one() {
        let co_m_bos = vec![
            "      3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "    1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "    8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "   1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "   6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let result = possible_games(co_m_bos, [12, 13, 14]);
        assert_eq!(result, 8);
    }
}
