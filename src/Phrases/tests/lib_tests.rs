#[cfg(test)]

mod tests
{
    extern crate phrases;
    // unit tests
    // Run 'cargo test' to execute unit tests
    #[test]
    //#[should_panic] // tells test runner to expect a failure
    //#[ignore] // ignores the test
    fn english_greeting_correct()
    {
        assert_eq!("hello", phrases::greetings::english::hello());
    }
}