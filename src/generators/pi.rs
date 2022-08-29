/*
    Do not use this for now until we find a way to use variable floating point precision

    Python has a library called `decimal` that let's you calculate to the Nth decimal, while Rust is
    confined to f32 and f64. For now I'll keep this here in case we do find something in the future.
*/

pub fn nth_pi(nth: i32) -> f64 {
    let mut c: f64 = 426880_f64 * 10005_f64.sqrt();
    let mut l: f64 = 13591409 as f64;
    let mut x: f64 = 1 as f64;
    let mut m: f64 = 1 as f64;
    let mut k: f64 = 6 as f64;
    let mut s: f64 = l as f64;
    for i in 1..nth {
        m = m * (k.powi(3) - 16_f64 * k) / f64::from(((i + 1).pow(3)));
        l += 545140134.0;
        x *= -262537412640768000.0;
        s += (m * l) / x;
    }
    let pi = c / s;
    println!("{}", pi);
    return pi;
}