fn main(){
    let input = include_str!("./input1.txt");
    let result = process(input);
    println!("{}", result);
}

fn process(input: &str) -> String{
    let lines: Vec<&str> = input.split("\n").collect();
    let mut values: Vec<i32> = vec![];

    for mut line in lines{
        line = line.trim();
        let (mut d1, mut d2) = (-1,-1);
        
        for char in line.chars(){
            if char.is_numeric(){
                if d1 == -1{
                    d1 = char.to_string().parse().unwrap();
                }else{
                    d2 = char.to_string().parse().unwrap();
                }
            }
        }

        if d2 == -1{
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
mod tests{
    use super::*;

    #[test]
    fn test_process(){
        let result = process("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result, "142".to_string());
    }
}