mod generators {
    pub mod num;
}

use std::io;

fn main() {
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

    let rand = generators::num::rand_num(floor_parsed, ceiling_parsed);

    println!("Your number is: {}", rand);
}
