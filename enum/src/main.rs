enum IpAddrKind {
    V4, 
    V6, 
}

// struct IpAddr {
//     kind: IpAddrKind, 
//     address: String, 
// }

// 型名にはstructで定義したものも使える
enum IpAddr {
    V4(String), 
    V6(String), 
}

#[derive(Debug)]
enum Message {
    Quit, 
    Move { x: i32, y: i32}, 
    Write(String), 
    ChangeColor(i32, i32, i32), 
}

// 関連したメソッドを定義することもできる
impl Message {
    fn show_this_message(&self) {
        println!("{:?}", self);
    }
}

// 内部で定義されているEnum
// enum Option<T> {
//     Some(T), 
//     None, 
// }

fn main() {
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;    

    // struct IpAddrを使ってこのようにも書けるが...
    // let home = IpAddr {
    //     kind: IpAddrKind::V4, 
    //     address: String::from("127.0.0.1"), 
    // };

    // enumでも対応する値を持たせられる
    let home = IpAddr::V4(String::from("127.0.0.1"));


    let mut message = Message::Quit;
    message.show_this_message();
    message = Message::Move { x: 30, y: 40 };
    message.show_this_message();
    message = Message::Write(String::from("Hello"));
    message.show_this_message();
    message = Message::ChangeColor(255, 255, 0);
    message.show_this_message();

    let mut maybe_number = Some(5);
    println!("{:?}", maybe_number);
    maybe_number = None;
    println!("{:?}", maybe_number);
}

fn route(ip_type: IpAddrKind) {
    // code...
}
