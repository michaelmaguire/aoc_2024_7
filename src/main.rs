use std::fs::File;
use std::io::{self, Read};

fn read_file_to_lines(path: &str) -> io::Result<Vec<String>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.lines().map(|line| line.to_string()).collect())
}
enum Operation {
    Add,
    Multiply,
    Concat,
}

fn try_combinations_of_operations(expected: u64, operands: &[u64]) -> usize {
    let mut count = 0;
    let n = operands.len();
    let operations = vec![Operation::Add, Operation::Multiply, Operation::Concat];
    let total_combinations = operations.len().pow((n - 1) as u32);

    for i in 0..total_combinations {
        let mut interim_result = operands[0];
        let mut combination = i;
        for j in 1..n {
            let operation = &operations[combination % operations.len()];
            combination /= operations.len();
            match operation {
                Operation::Add => interim_result += operands[j],
                Operation::Multiply => interim_result *= operands[j],
                Operation::Concat => {
                    let concat_str = format!("{}{}", interim_result, operands[j]);
                    interim_result = concat_str.parse::<u64>().unwrap_or(0);
                }
            }
        }
        if interim_result == expected {
            count += 1;
        }
    }

    count
}

fn parse_string(input: &str) -> Option<(u64, Vec<u64>)> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let first_part = parts[0].trim();
    let second_part = parts[1].trim();

    let first_number = first_part.parse::<u64>().ok()?;
    let numbers: Vec<u64> = second_part
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    Some((first_number, numbers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string_valid_input() {
        let input = "10: 1 2 3 4 5";
        let result = parse_string(input);
        assert_eq!(result, Some((10, vec![1, 2, 3, 4, 5])));
    }

    #[test]
    fn test_parse_string_invalid_input_no_colon() {
        let input = "10 1 2 3 4 5";
        let result = parse_string(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_string_invalid_input_non_numeric() {
        let input = "10: 1 2 three 4 5";
        let result = parse_string(input);
        assert_eq!(result, Some((10, vec![1, 2, 4, 5])));
    }

    #[test]
    fn test_parse_string_empty_input() {
        let input = "";
        let result = parse_string(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_string_only_colon() {
        let input = ":";
        let result = parse_string(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_string_extra_spaces() {
        let input = "  10  :  1  2  3  4  5  ";
        let result = parse_string(input);
        assert_eq!(result, Some((10, vec![1, 2, 3, 4, 5])));
    }
}

fn main() {
    println!("Hello, aoc_2024_7!");

    match read_file_to_lines("./src/input.txt") {
        Ok(lines) => {
            let mut sum_good_test_values = 0;
            for (_index, line) in lines.iter().enumerate() {
                //println!("Line {}: {}", index + 1, line);
                if let Some((expected, operands)) = parse_string(line) {
                    println!("Expected: {}, Operands: {:?}", expected, operands);
                    let number_of_good_combinations = try_combinations_of_operations(expected, &operands);
                    println!("Number of good combinations: {}", number_of_good_combinations);
                    if number_of_good_combinations > 0 {
                        sum_good_test_values += expected;
                    }
                } else {
                    println!("Failed to parse line: {}", line);
                }
                println!("sum_good_test_Values {sum_good_test_values}");
            }
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}
