// some examples 

// struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// struct method syntax
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// enum
struct Ipv4Addr {
    // snip
}

struct Ipv6Addr {
    // snip
}
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self)
    }
}

// match

fn main() {
    // structs, methods, associated fns, debug
    let rect1 = Rectangle {
        width: 50,
        height: 3,
    };
    let area = rect1.area();
    let square = Rectangle::square(4);
    println!("Can rect1 hold square?: {}", rect1.can_hold(&square));
    // debug
    println!("{:#?}", rect1); // pretty debug
    println!("{:?}", area); // debug

    // enum stuff
    let m = Message::Write(String::from("hello"));
    m.call();

    let l = Message::ChangeColor(24,24,24);
    l.call();

    // Option enums
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    println!("{:?}",(x + some_number.unwrap())); // unwrap to access value
    println!("{:?}",(some_number)); // Some(5)
    println!("{:?}",(absent_number)); // None
}
