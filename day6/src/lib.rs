
#[derive(PartialEq, Eq, Debug)]
pub struct Record {
    time_ms: i32,
    distance: i32,
}

pub fn parse(data: String) -> Result<Vec<Record>, &'static str> {    
    let lines: Vec<&str> = data.split("\n").collect();
    if lines.len() != 2{
        return Err("Invalid number of inputs");
    }
    let (times, distances) = (lines[0].trim(), lines[1].trim());

    let times: Vec<i32> = times.split_whitespace().into_iter().skip(1).map(|x| x.parse::<i32>().unwrap()).collect();
    let distances: Vec<i32> = distances.split_whitespace().into_iter().skip(1).map(|x| x.parse().unwrap()).collect();

    if times.len() != distances.len(){
        return Err("Times and Distances not matching")
    }

    // if we made it this far we have all we need to pair up our results
    let combo = times.iter().zip(distances.iter());
    let mut res: Vec<Record> = Vec::with_capacity(times.len());
    for (x, y) in  combo{
        res.push(Record { time_ms: *x, distance: *y });
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn base_test() {
        assert_eq!(1, 1);
    }

    // todo: make a test for reading string to record vector
    #[test]
    fn test_parsing(){
        let input = "Time:      7  15   30
        Distance:  9  40  200".to_string();

        let result = parse(input).expect("should have parsed");

        assert_eq!(result,
            vec![
                Record{time_ms: 7,distance: 9},
                Record{time_ms: 15,distance: 40},
                Record{time_ms: 30,distance: 200}
            ]
        )

    }

    // todo: make a test for calculating possible solutions
}
