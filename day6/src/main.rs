use day6;


fn main() {
    let res: String = "Time:        50     74     86     85
    Distance:   242   1017   1691   1252".to_string();
    let values = day6::parse(res).expect("Should have gotten values");
    let product: i32 = values.into_iter().map(|x| day6::count_possibilities(&x)).product();

    println!("Calculated {} as result for part1.", product);

}
