
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

pub fn count_possibilities(record: &Record) -> i32{
    // the way we solve is we count how many seconds there are,
    // then for i in seconds
    //   calculate: remaining seconds, current speed, potential distance traveled at current speed (delta)
    //   if: delta is larger than current record distance, count ++

    let mut count = 0;
    for i in 1..record.time_ms{
        let remaining_seconds = record.time_ms - i;
        let potential_distance_traveled = i * remaining_seconds;
        if potential_distance_traveled > record.distance{
            count+= 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn base_test() {
        assert_eq!(1, 1);
    }

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
    #[test]
    fn test_count_possibilities(){
        let input = Record{ time_ms: 7, distance: 9};
        assert_eq!(4, count_possibilities(&input));

        let input = Record{ time_ms: 15, distance: 40};
        assert_eq!(8, count_possibilities(&input));

        let input = Record{ time_ms: 30, distance: 200};
        assert_eq!(9, count_possibilities(&input));
    }
}
