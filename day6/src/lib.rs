#[derive(PartialEq, Eq, Debug)]
pub struct Record {
    time_ms: usize,
    distance: usize,
}

pub fn parse(data: String) -> Result<Vec<Record>, &'static str> {
    let lines: Vec<&str> = data.split("\n").collect();
    if lines.len() != 2 {
        return Err("Invalid number of inputs");
    }
    let (times, distances) = (lines[0].trim(), lines[1].trim());

    let times: Vec<usize> = times
        .split_whitespace()
        .into_iter()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let distances: Vec<usize> = distances
        .split_whitespace()
        .into_iter()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    if times.len() != distances.len() {
        return Err("Times and Distances not matching");
    }

    // if we made it this far we have all we need to pair up our results
    let combo = times.iter().zip(distances.iter());
    let mut res: Vec<Record> = Vec::with_capacity(times.len());
    for (x, y) in combo {
        res.push(Record {
            time_ms: *x,
            distance: *y,
        });
    }

    Ok(res)
}

pub fn parse_part2(input: String) -> Result<Record, &'static str> {
    let lines: Vec<&str> = input.split("\n").collect();
    if lines.len() != 2 {
        return Err("Invalid number of inputs");
    }
    let (times, distances) = (lines[0].trim(), lines[1].trim());

    let time: usize = times
        .split_whitespace()
        .into_iter()
        .skip(1)
        .collect::<Vec<&str>>()
        .concat()
        .parse::<usize>()
        .unwrap();
    let distance: usize = distances
        .split_whitespace()
        .into_iter()
        .skip(1)
        .collect::<Vec<&str>>()
        .concat()
        .parse::<usize>()
        .unwrap();

    Ok(Record {
        time_ms: time,
        distance,
    })
}

pub fn count_possibilities(record: &Record) -> usize {
    // the way we solve is we count how many seconds there are,
    // then for i in seconds
    //   calculate: remaining seconds, current speed, potential distance traveled at current speed (delta)
    //   if: delta is larger than current record distance, count ++

    let mut count = 0;
    for i in 1..record.time_ms {
        let remaining_seconds = record.time_ms - i;
        let potential_distance_traveled = i * remaining_seconds;
        if potential_distance_traveled > record.distance {
            count += 1;
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
    fn test_parsing() {
        let input = "Time:      7  15   30
        Distance:  9  40  200"
            .to_string();

        let result = parse(input).expect("should have parsed");

        assert_eq!(
            result,
            vec![
                Record {
                    time_ms: 7,
                    distance: 9
                },
                Record {
                    time_ms: 15,
                    distance: 40
                },
                Record {
                    time_ms: 30,
                    distance: 200
                }
            ]
        )
    }

    #[test]
    fn test_parse_part2() {
        let input = "Time:      7  15   30
        Distance:  9  40  200"
            .to_string();

        let result = parse_part2(input).unwrap();

        assert_eq!(
            result,
            Record {
                time_ms: 71530,
                distance: 940200
            }
        );
    }

    // todo: make a test for calculating possible solutions
    #[test]
    fn test_count_possibilities() {
        let input = Record {
            time_ms: 7,
            distance: 9,
        };
        assert_eq!(4, count_possibilities(&input));

        let input = Record {
            time_ms: 15,
            distance: 40,
        };
        assert_eq!(8, count_possibilities(&input));

        let input = Record {
            time_ms: 30,
            distance: 200,
        };
        assert_eq!(9, count_possibilities(&input));
    }
}
