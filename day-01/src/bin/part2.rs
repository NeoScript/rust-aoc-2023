fn main(){
    println!("part 2 running.");
}

fn process(input: &str) -> String{
    "result2".to_string()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_process(){
        let result = process("value");
        assert_eq!(result, "result2".to_string());
    }
}