// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
}

#[cfg(test)]
mod tests {
 use super::*;   
    #[test]
    fn call_enum() {
        println!("{:?}", Message::Quit);
        println!("{:?}", Message::Echo);
        println!("{:?}", Message::Move);
        println!("{:?}", Message::ChangeColor);    
    }

}

