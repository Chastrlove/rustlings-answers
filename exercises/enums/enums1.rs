// enums1.rs
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,
    // TODO: define a few types of messages as used below
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
