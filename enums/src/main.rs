fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    let _home = IpAddrKind::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}

// You can put any kind of data inside an
// enum variant, even another enum.
enum IpAddrKind {
    V4(String), // have a String associated
    V6(String),
}

enum Message {
    Quit,                       // no data associated
    Move { x: i32, y: i32 },    // anonymous struct
    Write(String),              // includes a String
    ChangeColor(i32, i32, i32), // includes three i32 values
}

// We're able to define methods on enums
impl Message {
    fn call(&self) {
        // method body here
    }
}
