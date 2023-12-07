fn main(){
    println!("part 1 running.");
}

fn process(input: &str) -> String{
    "result1".to_string()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_process(){
        let result = process("value");
        assert_eq!(result, "result1".to_string());
    }
}