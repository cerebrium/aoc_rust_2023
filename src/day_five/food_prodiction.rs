fn food_production(data: Vec<Vec<&str>>, seeds: &str) -> i64 {
    /*
        Each line within a map contains three numbers:
            1. The destination range start
            2. The source range start
            3. The range length

    Ex:
        seed-to-soil: 50 98 2 ->
            - The destination (soil) is 50
            - The source (seed) is 98
            - The seed (98) matches the soil (50)
            - The seed (99) matches the soil (51)
            - This is the first line

        Second line has rules for its' range of numbers

        Anything not listed, maps 1:1

    */

    let mut range_finder = vec![];
    let mut destination_finder = vec![];

    for (map_i, map) in data.iter().enumerate() {
        destination_finder.push(vec![]);
        range_finder.push(vec![]);

        for range in map {
            let nums: Vec<&str> = range.split(" ").collect();

            let mut source_tuple: Vec<i64> = vec![0, 0];
            let mut first_num = 0;

            for (idx, num) in nums.iter().enumerate() {
                // this is the destination
                if idx == 0 {
                    println!("first num before: {:?}", num);
                    first_num = num.parse::<i64>().unwrap();
                    println!("first num: {:?}", first_num);
                }

                // This is the source
                if idx == 1 {
                    source_tuple[0] = num.parse::<i64>().unwrap();
                    destination_finder[map_i].push(first_num - source_tuple[0]);
                    continue;
                }

                // this is the range
                if idx == 2 {
                    source_tuple[1] = source_tuple[0] + num.parse::<i64>().unwrap();

                    continue;
                }
            }

            range_finder[map_i].push(source_tuple);
        }
    }

    let mut final_seeds: Vec<i64> = vec![];
    let parsed_seeds: Vec<&str> = seeds.split(" ").collect();
    for seed in parsed_seeds {
        let mut seed_num = seed.parse::<i64>().unwrap();

        for (range_i, range) in range_finder.iter().enumerate() {
            for (match_i, potential_match) in range.iter().enumerate() {
                if seed_num >= potential_match[0] && seed_num < potential_match[1] {
                    let operation = destination_finder[range_i][match_i];

                    seed_num += operation;
                    break;
                }
            }
        }

        final_seeds.push(seed_num);
    }

    *final_seeds.iter().min().unwrap()
}

mod tests {
    use super::*;

    #[test]

    fn test_food_production() {
        let test_data = vec![
            vec!["50 98 2", "52 50 48"],
            vec!["0 15 37", "37 52 2", "39 0 15"],
            vec!["49 53 8", "0 11 42", "42 0 7", "57 7 4"],
            vec!["88 18 7", "18 25 70"],
            vec!["45 77 23", "81 45 19", "68 64 13"],
            vec!["0 69 1", "1 0 69"],
            vec!["60 56 37", "56 93 4"],
        ];

        let seeds = "79 14 55 13";
        let result = food_production(test_data, seeds);

        assert_eq!(result, 35);
    }
}
