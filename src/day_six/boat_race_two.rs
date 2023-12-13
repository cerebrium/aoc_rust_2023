/*

    Time:      7  15   30
    Distance:  9  40  200
    This document describes three races:

    The first race lasts 7 milliseconds. The record distance in this race is 9 millimeters.
    The second race lasts 15 milliseconds. The record distance in this race is 40 millimeters.
    The third race lasts 30 milliseconds. The record distance in this race is 200 millimeters.

*/

fn boat_race_two(data: Vec<&str>) -> usize {
    let mut total_winning_durations = 0;

    let time = data[0].parse::<usize>().unwrap();
    let distance_to_beat = data[1].parse::<usize>().unwrap();

    for i in 1..time - 1 {
        if find_distance(i, time) > distance_to_beat {
            total_winning_durations += 1;
        }
    }

    total_winning_durations
}

fn find_distance(hold_time: usize, total_time: usize) -> usize {
    (total_time - hold_time) * hold_time
}

mod tests {
    use super::*;

    #[test]

    fn test_boat_race_two() {
        let test_data = vec!["71530", "940200"];
        let result = boat_race_two(test_data);
        assert_eq!(result, 71503);
    }
}
