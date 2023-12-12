pub fn calibrate_one(strs: Vec<&str>) -> usize {
    let mut sum: usize = 0;
    for line in strs {
        let mut first = 'n';
        let mut last = 'n';

        for char in line.chars() {
            if char.is_numeric() {
                if first == 'n' {
                    first = char;
                } else {
                    last = char;
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
    fn cal_one() {
        let input = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

        let result = calibrate_one(input);
        assert_eq!(result, 142);
    }

*/
