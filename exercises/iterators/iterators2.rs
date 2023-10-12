// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.


// 使用typename包
use std::any::type_name;

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    // 创建字符迭代器  将string 拆分成一个一个char的迭代器
    let mut c = input.chars();
    // 获取迭代器第一个元素
    match c.next() {
        None => String::new(),
        Some(first) => {
            // 将首字母转大写
           let mut result =first.to_uppercase().to_string();
            // 拼接后面的字符
           result.extend(c);
           result
        }

    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // 对迭代器闭包进行广播操作  然后收集到列表里面
    words.iter().map(|&word| capitalize_first(word)).collect()  //ok
    
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    // String::new()
    // 不会对非字母字符进行匹配，直接复制即可
    words.iter().map(|&word| capitalize_first(word)).collect()  //ok
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
