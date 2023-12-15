use day6;


fn main() {
    let input: String = "Time:        50     74     86     85
    Distance:   242   1017   1691   1252".to_string();
    let values = day6::parse(input.clone()).expect("Should have gotten values");
    let product: usize = values.into_iter().map(|x| day6::count_possibilities(&x)).product();
    println!("Calculated {} as result for part1.", product);

    let record2 = day6::parse_part2(input.clone()).unwrap();
    let possibilities2 = day6::count_possibilities(&record2);
    println!("Part2: {}", possibilities2);
}
