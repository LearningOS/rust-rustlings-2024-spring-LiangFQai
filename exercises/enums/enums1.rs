// enums1.rs
//
// No hints this time! ;)




#[derive(Debug)]
enum Message {
    Quit,
    Echo { x: i32, y: i32 },
    Move(i32, i32),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // 创建不同成员的实例，并打印出来
    let quit_message = Message::Quit;
    let echo_message = Message::Echo { x: 1, y: 2 };
    let move_message = Message::Move(3, 4);
    let change_color_message = Message::ChangeColor(5, 6, 7);

    println!("{:?}", quit_message);
    println!("{:?}", echo_message);
    println!("{:?}", move_message);
    println!("{:?}", change_color_message);
}

