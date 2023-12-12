#[cfg(test)]

fn possible_games_two(combos: Vec<&str>) -> usize {
    use std::collections::HashMap;

    let mut sum = 0;
    let mut maxes = HashMap::from([]);

    for line in combos {
        let mut local_sum = 1;

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

                if let Some(current_color_sum) = maxes.get(&first_letter_of_color) {
                    if current_color_sum < &amount {
                        maxes.insert(first_letter_of_color, amount);
                    }
                } else {
                    maxes.insert(first_letter_of_color, amount);
                }
            });
        });

        maxes.values().for_each(|v| {
            local_sum *= v;
        });
        sum += local_sum;
        maxes = HashMap::from([]);
    }

    sum as usize
}

mod tests {

    use super::*;

    #[test]
    fn cal_two() {
        let test_data = vec![
            "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        println!("is this running?");

        let result = possible_games_two(test_data);
        assert_eq!(result, 2286);
    }
}
