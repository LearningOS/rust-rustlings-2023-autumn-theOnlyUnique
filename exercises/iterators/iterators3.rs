// iterators3.rs
//
// This is a bigger exercise than most of the others! You can do it! Here is
// your mission, should you choose to accept it:
// 1. Complete the divide function to get the first four tests to pass.
// 2. Get the remaining tests to pass by completing the result_with_list and
//    list_of_results functions.
//
// Execute `rustlings hint iterators3` or use the `hint` watch subcommand for a
// hint.



#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    // todo!();
    if b==0 {
       return  Err(DivisionError::DivideByZero);
    }
    else if (a%b==0) {
        Ok(a/b)
    }
    else{
        // 除数不可除
        return Err(DivisionError::NotDivisible(NotDivisibleError {
            dividend: a,
            divisor: b
        }));
    }
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: Ok([1, 11, 1426, 3])
// 注意观察内层的嵌套顺序 和结果的顺序
fn result_with_list() -> (Result<Vec<i32>, DivisionError>) {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27)).collect();
    division_results
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
// 这里的类型要和divide的返回类型保持对应
fn list_of_results() -> (Vec<Result<i32, DivisionError>>) {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27)).collect(); //这里也可以显式指定collect收集的类型 只要你返回值写对了 collect就给你收集对应的类型
    
    division_results
    // division_results.to_string()
}
// summary
// 将结果收集起来组成列表
// [Ok(1), Ok(11), Ok(1426), Ok(3)]
// iter.map(|x| fun(x)).collect()
// 将中间结果收集起来 组成一个Ok(这个是错误的)
// Ok([1, 11, 1426, 3])
// iter.map(|x| fun(x).collect())

// tips:

// collect() 方法可以根据上下文自动推断要收集的类型，
// 而 collect::<T>() 允许你显式指定要收集到的类型 T。
// 例如，collect::<Vec<_>>() 显式指定了要收集到 Vec 类型，
// 而 collect() 会自动根据使用场景来推断要收集的类型。
#[cfg(test)]
mod tests {
    use super::*;
    // 1
    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }
    // 2
    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }
    // 3
    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }
    // 4
    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }
    // 5
    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }
    // 6
    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
