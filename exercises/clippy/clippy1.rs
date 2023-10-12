// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.



use std::f32;
// Clippy提示使用f32::consts::PI常量而不是手动定义一个近似值。
fn main() {
    let pi = f32::consts::PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);
    // let area = pi * f32::powf(radius, 2.0);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}