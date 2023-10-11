// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.



#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        if value>=0 {
            if value>0 {
                Ok(PositiveNonzeroInteger(value as u64))
            }
            else {
                Err(CreationError::Zero)
            }

        }else{
            Err(CreationError::Negative)
        }
    }
}

#[test]
fn test_creation() {
    // Rust的断言宏，用于在单元测试中检查条件是否为真
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
