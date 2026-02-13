fn main() {
    println!("attribute is the meta data for a code snippet");
    println!("`cargo test` to run all tests in project");
    println!("`cargo test  -- --test-threads=1 --show-output`");
    println!("`cargo test add` to test all tests whose name contains add");
    println!("`cargo test add -- --exact` to run test whose name is add");
    println!("`cargo test -- --ignored` to run ignored tests");
    println!("`cargo test -- --include-ignored` to run all tests");
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

// unit test
#[cfg(test)] // only build when test configuration
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "equal to 4")] // should panic(substring)
    fn exploration() {
        let result = add(2, 2);
        // assert! macro: panic if false
        assert!(result > 0, "result {result} must be greater than 0");
        // assert_eq! macro: should impl Debug and PartialEq
        assert_eq!(result, 4);
        assert_ne!(result, 4, "should equal to 4");
    }

    #[test]
    #[ignore]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("add(2, 2) should equal to 4"))
        }
    }
}
