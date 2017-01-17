use std::io;

/*enum Option<T> {
    None,
    Some(T)
}*/

fn safe_div(n: i32, d: i32) -> Option<i32> {
    if d == 0 {
        return None;
    }
    return Some(n/d);
}

fn main() {
    println!("Guess the number!");
    println!("Please input your numerator.");
    let mut numerator = String::new();
    io::stdin().read_line(&mut numerator)
        .expect("Failed to read line");
    let numerator_int = numerator.parse::<i32>().unwrap();
    println!("Please input your denominator.");
    let mut denominator = String::new();
    io::stdin().read_line(&mut denominator)
        .expect("Failed to read line");
    let denominator_int = denominator.parse::<i32>().unwrap();
    match safe_div(numerator_int, denominator_int) {
        None => println!("Can't divide by zero!"),
        Some(v) => println!("Quotient is {}", v)
    }
}
