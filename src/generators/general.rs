//! Description: Generators used to supply commands with information

use rand::Rng;
use std::collections::HashMap;

pub fn nth_fibo(mut nth: i32) -> i32 {
    if nth == 0 {
        return 1;
    } else if nth == 1 {
        return 2;
    } else {
        let mut num1 = 0;
        let mut num2 = 1;

        while nth > 1 {
            let next: u32 = num1 + num2;

            num1 = num2;
            num2 = next;

            nth -= 1;
        }
        return num2.try_into().unwrap();
    }
}

pub fn rand_num(floor: i32, ceiling: i32) -> i32 {
    return rand::thread_rng().gen_range(floor..=ceiling);
}

pub fn eight_ball() -> &'static str {
    let responses = [
        "It is certain",
        "Without a doubt",
        "You may rely on it",
        "Yes, definitely",
        "It is decidedly so",
        "As I see it, yes",
        "Most likely",
        "Yes",
        "Outlook good",
        "Signs point to yes",
        "Reply hazy try again",
        "Better not tell you now",
        "Ask again later",
        "Cannot predict now",
        "Concentrate and ask again",
        "Don't count on it",
        "Outlook not so good",
        "My sources say no",
        "Very doubtful",
        "My reply is no",
    ];
    let index = rand::thread_rng().gen_range(0..=responses.len() - 1);
    return responses[index];
}

pub fn word() -> (String, String) {
    let config_path = "dictionary.json";
    let config_read = std::fs::read_to_string(&config_path);

    let dictionary: HashMap<String, String> = serde_json::from_str(&config_read.unwrap()).unwrap();

    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();

    for (key, value) in dictionary {
        keys.push(key);
        values.push(value);
    }

    let choice = rand::thread_rng().gen_range(0..keys.len() - 1);

    let keys_ret = &keys[choice];
    let values_ret = &values[choice];

    return (String::from(keys_ret), String::from(values_ret));
}

// TODO: Figure out a way to fix this in regards to floating point stuff
// pub fn nth_pi(nth: i32) -> f64 {
//     /// Returns the Nth digit of Pi giving the nth argument
//     ///
//
//     /// Do not use this for now until we find a way to use variable floating point precision
//     ///
//     /// Python has a library called `decimal` that let's you calculate to the Nth decimal, while Rust is
//     /// confined to f32 and f64. For now I'll keep this here in case we do find something in the future.
//     let mut c: f64 = 426880_f64 * 10005_f64.sqrt();
//     let mut l: f64 = 13591409 as f64;
//     let mut x: f64 = 1 as f64;
//     let mut m: f64 = 1 as f64;
//     let mut k: f64 = 6 as f64;
//     let mut s: f64 = l as f64;
//     for i in 1..nth {
//         m = m * (k.powi(3) - 16_f64 * k) / f64::from(((i + 1).pow(3)));
//         l += 545140134.0;
//         x *= -262537412640768000.0;
//         s += (m * l) / x;
//     }
//     let pi = c / s;
//     println!("{}", pi);
//     return pi;
// }
