//! Description: Unit tests to test functions in main.rs


#[cfg(test)]
mod tests {
    use crate::check_dev;

    #[test]
    fn check_dev_test_true() {
        let result = check_dev("514866599400833034".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn check_dev_test_false() {
        let result = check_dev("1234567890".to_string());
        assert_eq!(result, false)
    }
}