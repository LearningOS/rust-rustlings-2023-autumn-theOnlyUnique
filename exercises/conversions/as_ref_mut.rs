// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.



// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn byte_counter<T:AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn char_counter<T:AsRef<str>>(arg: T) -> usize {
    arg.as_ref()   //转成 &str引用
    .chars()        //产生迭代器
    .count()        //统计数量
}

// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.
// as_mut() 方法的目的是获取参数的可变引用
fn num_sq<T:AsMut<u32>>(arg: &mut T) {
    // TODO: Implement the function body.
    let availableValue =arg.as_mut();
    (*availableValue)=(*availableValue).pow(2)
}
// 当你调用 .pow(2) 方法时，它期望一个拥有所有权的整数类型，
// 而 &mut T 类型是一个可变引用，而不是拥有所有权的值。
// (*availableValue).pow(2) 试图在 &mut T 上执行 .pow(2) 操作，这是不合法的，因为这个操作需要值的所有权，而 &mut T 只是一个引用。

// 所以，你需要显式解引用 availableValue，获取它的值，并执行 .pow(2)，
// 然后再将结果赋回 availableValue，以便操作的是值而不是引用。这样才能符合 Rust 的借用规则，保证安全性。
// 这里没有太看懂 只知道这样是为了只操作值 避免对引用的操作？？？？？？？
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
