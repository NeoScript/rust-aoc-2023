fn main() {
    let input = include_str!("./input2.txt");
    let result = process(input);
    println!("{}", result);
}

fn reformat_line(line: &str) -> String {
    let mut value = line.trim().to_string();
    value = value
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9");

    value.to_string()
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
    fn test_reformat_line(){
        let input = "one2three";
        let result = reformat_line(input);

        assert_eq!(result, "123".to_string());
    }

    #[test]
    fn test_reformat_tricky_line(){
        let input = "eightwo1";
        let result = reformat_line(input);

        assert_eq!(result, "8wo1".to_string());
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
