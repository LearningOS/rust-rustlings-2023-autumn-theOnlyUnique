// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.



// 不是很清楚这里为什要用引用  而不是直接传进去
// clone 方法实际上会创建一个新的引用计数智能指针，
// 它引用与原始引用相同的数据，并且增加引用计数。
// 这不会创建数据的实际副本，而只是增加数据的引用计数。
// 引用计数会跟踪多少个引用指向相同的数据，当引用计数达到零时，数据会被释放。

// 所以clone实际上是使用的指向同一个变量的指针
#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers =Arc::new(numbers); //共享变量
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = shared_numbers.clone();  //不使用clone方法会移交所有权
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
