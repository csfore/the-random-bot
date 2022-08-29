use std::io;
use rand::Rng;

pub fn rand_num(floor: i32, ceiling: i32) -> i32 {
    return rand::thread_rng().gen_range(floor..=ceiling);
}