// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.



#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}
// tips:
// 1.宏的规则分支之间需要使用分号或大括号进行分隔，以区分不同的规则。
fn main() {
    my_macro!();
    my_macro!(7777);
}
