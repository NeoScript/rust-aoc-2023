use std::{fs, vec};

pub struct Record {
    time_ms: i32,
    distance: i32,
}

pub fn parse(data: String) -> Result<Vec<Record>, &'static str> {    
    let lines: Vec<&str> = data.split("\n").collect();
    if lines.len() != 2{
        return Err("Invalid number of inputs");
    }
    let (times, distances) = (lines[0], lines[1]);

    let times: Vec<i32> = times.split_whitespace().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let distances: Vec<i32> = distances.split_whitespace().into_iter().map(|x| x.parse().unwrap()).collect();

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

    #[test]
    fn base_test() {
        assert_eq!(1, 1);
    }

    // todo: make a test for reading string to record vector

    // todo: make a test for calculating possible solutions
}
