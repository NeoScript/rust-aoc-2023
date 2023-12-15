use day6;


fn main() {
    let res: String = "Time:      7  15   30
    Distance:  9  40  200".to_string();
    let values = day6::parse(res).expect("Should have gotten values");
    
    println!("Hello, world!");
}
