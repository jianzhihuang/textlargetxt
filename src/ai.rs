pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    let num = args[1].parse::<i32>().unwrap();
    println!("{:?}", num);
}
