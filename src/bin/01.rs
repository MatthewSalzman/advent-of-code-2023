advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    for value in input.split("\n").into_iter() {
        sum += get_calibration_value(value) as u32;
    }
    Some(sum)
}

const NUMBER_MAP: [(&'static str, &str); 10] = [
    ("zero", "0"), 
    ("one", "1"), 
    ("two", "2"), 
    ("three", "3"), 
    ("four", "4"), 
    ("five", "5"), 
    ("six", "6"), 
    ("seven", "7"), 
    ("eight", "8"), 
    ("nine", "9"), 
];

pub fn part_two(input: &str) -> Option<u32> {
    // let input = get_numbers_from_input(input);
    let mut sum: u32 = 0;
    for value in input.split("\n").into_iter() {
        let converted_input = get_numbers_from_input(value);
        sum += get_calibration_value(&converted_input) as u32;
    }
    
    Some(sum)
}

/// Get the calibration value from a given string.
fn get_calibration_value(input: &str) -> u8 {


    // Get all the numbers from input
    let nums: Vec<char> = input.chars().filter(|c| c.is_numeric()).collect();

    let output = format!("{}{}",nums.first().unwrap().to_string(), nums.last().unwrap().to_string());
    // println!("{}", output);

    output.parse().unwrap()
    
}


fn get_numbers_from_input(input: &str) -> String {
    // let mut input = input.to_owned();

    let mut numbers_array: Vec<(u8, String)> = vec![];

    // Get digits out
    for (idx, c) in input.chars().enumerate() {
        if c.is_numeric() {
            numbers_array.push((idx as u8, c.clone().to_string()))
        }
    }


    // Get string digits out
    for (stringnum, num) in NUMBER_MAP {
        let idxs: Vec<(usize, &str)> = input.match_indices(stringnum).collect();
        for (x,y) in idxs {
            numbers_array.push((x as u8, y.replace(stringnum, num).clone().to_string()))
        }
    }

    // Sort array to have indexes first
    numbers_array.sort_by_key(|a| a.0);

    let mut res = String::new();
    for (_x,y ) in numbers_array {
        res.push_str(&y)
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55002));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55093));
    }

    #[test]
    fn test_get_calibration_value() {
        let input = "1abc2";
        let value = get_calibration_value(input);
        assert_eq!(value, 12);

        let input = "pqr3stu8vwx";
        let value = get_calibration_value(input);
        assert_eq!(value, 38);

        let input = "a1b2c3d4e5f";
        let value = get_calibration_value(input);
        assert_eq!(value, 15);

        let input = "treb7uchet";
        let value = get_calibration_value(input);
        assert_eq!(value, 77);


    }


    #[test]
    fn test_text_numers_to_numbers() {
        assert_eq!("219", get_numbers_from_input("two1nine"));

        assert_eq!("823", get_numbers_from_input("eightwothree"));

        assert_eq!("123", get_numbers_from_input("abcone2threexyz"));

    }

}
