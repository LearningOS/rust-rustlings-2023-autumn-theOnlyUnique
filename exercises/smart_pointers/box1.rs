// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: the value of the current item and
// the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.



#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(0, Box::new(List::Nil))
}

// summary
// 1.assert_ne!()函数  检查两个值是否不相等
// 2.关于box
// 在这个测试中，你需要使用 Box 来解决递归类型的问题。Rust 在编译时需要知道每种类型占用的空间大小，但递归类型的大小是不定的，因为一个类型的值包含了另一个相同类型的值，这会导致无限大小。所以，你需要使用 Box 来分配递归类型的内存到堆上，这允许 Rust 知道所有 List 类型的大小。

// 递归类型就像链表数据结构中的节点，其中每个节点都包含了一个值和指向下一个节点的指针。由于不知道递归类型有多大，使用 Box 来将每个节点分配到堆上，而不是直接放在栈上，从而解决了问题。这使得 Rust 能够准确地知道每个节点的大小，因为每个 Box 指针的大小是固定的，而不是变化的。

// 使用 Box 可以让递归类型在 Rust 中得以实现，同时确保不会出现无限递归和栈溢出问题。这种方式在处理树形或链表结构等递归数据类型时非常有用。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
