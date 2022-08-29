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