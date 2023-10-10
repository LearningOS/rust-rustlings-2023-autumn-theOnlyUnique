// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo(String),
    Move {x:i32,y:i32},
    ChangeColor {r:u8,g:u8,b:u8}
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("hello")));
    println!("{:?}", Message::Move{x:50,y:100});
    println!("{:?}", Message::ChangeColor{r:0,g:255,b:0});
}
