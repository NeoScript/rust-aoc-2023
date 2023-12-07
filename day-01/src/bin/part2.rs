fn main() {
    let input = include_str!("./input2.txt");
    let result = process(input);
    println!("{}", result);
}

fn reformat_line(line: &str) -> String {
    let mut value = line.trim().to_string();
    // need to use a window instead of basic replace
    // create two pointers
    // start & end
    // while end is not larger than the string length
    // check if substring from start..end is replaceable
    // if it is, then replace it
    // set start to end + 1
    //  continue till end
    // return new string
    let mut start = 0;
    let mut end = 1;

    while end < value.len() {
        let current_word = value[start..=end].to_string();
        let new_word = current_word
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");

        if current_word != new_word {
            let pre = &value[..start];
            let post = &value[(end + 1)..];

            value = format!("{}{}{}", pre, new_word, post);
            start = start + 1;
            end = start + 1;
        } else {
            end = end + 1;
        }
    }

    value
}

fn process(input: &str) -> String {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut values: Vec<i32> = vec![];

    for line in lines {
        let line = reformat_line(line);
        let (mut d1, mut d2) = (-1, -1);

        for char in line.chars() {
            if char.is_numeric() {
                if d1 == -1 {
                    d1 = char.to_string().parse().unwrap();
                } else {
                    d2 = char.to_string().parse().unwrap();
                }
            }
        }

        if d2 == -1 {
            d2 = d1;
        }

        let line_value = (d1 * 10) + d2;
        values.push(line_value)
    }

    println!("values -> {:?}", values);
    let sum: i32 = values.iter().sum();
    let result = format!("{}", sum);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reformat_line() {
        let input = "one2three";
        let result = reformat_line(input);

        assert_eq!(result, "o1e2t3e".to_string());
    }

    #[test]
    fn test_reformat_tricky_line() {
        let input = "eightwo1";
        let result = reformat_line(input);

        assert_eq!(result, "e8t2o1".to_string());
    }

    #[test]
    fn test_nonsense_format() {
        // how tf is this day one
        let input = "eightwo";
        let result = reformat_line(input);

        assert_eq!(result, "e8t2o".to_string());
    }

    #[test]
    fn test_process() {
        let result = process(
            "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen",
        );
        assert_eq!(result, "281".to_string());
    }
}
