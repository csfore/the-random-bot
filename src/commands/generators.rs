use rand::Rng;


extern crate wikipedia;


pub fn nth_fibo(mut nth: i32) -> i32 {
    /// Gets the Nth digit in the Fibonacci sequence

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
    /// Returns a random number from the floor to the ceiling

    return rand::thread_rng().gen_range(floor..=ceiling);
}

// pub fn nth_pi(nth: i32) -> f64 {
//     /// Returns the Nth digit of Pi giving the nth argument
//     ///
//     /// TODO: Figure out a way to fix this in regards to floating point stuff
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


// pub fn wiki() -> String {
//     let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
//     let page = wiki.random();
//
//     // let wiki_vec = WrapperWiki {
//     //     result: page
//     // };
//     let t = page.unwrap().expect("Something");
//
//     let result = wiki.page_from_title(t);
//     let content = result.get_content().unwrap();
//
//
//     println!("")
// }