mod generators {
    pub mod num;
    pub mod fibo;
}

use std::io;

fn main() {
    // This is all for debugging purposes
    let mut floor = String::new();
    let mut ceiling = String::new();

    println!("Enter your floor:");
    io::stdin()
        .read_line(&mut floor)
        .expect("Didn't get that");

    println!("Enter your ceiling: ");
    io::stdin()
        .read_line(&mut ceiling)
        .expect("Didn't get that");

    let floor_parsed: i32 = floor.trim().parse().unwrap();
    let ceiling_parsed: i32 = ceiling.trim().parse().unwrap();

    let fibo = generators::fibo::nth_fibo(floor_parsed);

    println!("Nth digit is: {}", fibo);
}
