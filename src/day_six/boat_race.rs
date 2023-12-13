/*

    Time:      7  15   30
    Distance:  9  40  200
    This document describes three races:

    The first race lasts 7 milliseconds. The record distance in this race is 9 millimeters.
    The second race lasts 15 milliseconds. The record distance in this race is 40 millimeters.
    The third race lasts 30 milliseconds. The record distance in this race is 200 millimeters.

*/

fn boat_race(data: Vec<&str>) -> usize {
    let mut total_winning_durations = 0;
    let times: Vec<usize> = data[0]
        .split(" ")
        .map(|t| t.parse::<usize>().unwrap())
        .collect();

    let distances: Vec<usize> = data[1]
        .split(" ")
        .map(|d| d.parse::<usize>().unwrap())
        .collect();

    for (i, time) in times.iter().enumerate() {
        let mut winning_durations = 0;

        let distance_to_beat = distances[i];

        for i in 1..time - 1 {
            if find_distance(i, *time) > distance_to_beat {
                winning_durations += 1;
            }
        }

        if winning_durations == 0 {
            continue;
        }
        if total_winning_durations == 0 {
            total_winning_durations = winning_durations;
        } else {
            total_winning_durations *= winning_durations;
        }
        println!("what is the time: {:?}", time);
    }

    total_winning_durations
}

fn find_distance(hold_time: usize, total_time: usize) -> usize {
    (total_time - hold_time) * hold_time
}

mod tests {
    use super::*;

    #[test]

    fn test_boat_race() {
        let test_data = vec!["7 15 30", "9 40 200"];
        let result = boat_race(test_data);
        assert_eq!(result, 288);
    }
}
