fn main() {
    println!("Hello, world!");
}

// 1. enum
enum IpAddrKind {
    V4,
    V6,
}
fn enum_1() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    call_1(four);
    call_1(six);
    call_1(IpAddrKind::V6);
}

fn call_1(ip_kind: IpAddrKind) {}

// 2. 与 struct 一起使用
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn enum_2() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

// 3. 将数据附加到枚举的变体中

enum IpAddr1 {
    V4(String),
    V6(String),
}
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String), // 也可以定义自定义的结构体类型
}

fn enum_3() {
    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));
}

// 枚举也可以定义方法
impl IpAddr1 {
    fn call(&self) {}
}

// 4. 范型参数
// enum Option<T> {
//     Some(T),
//     None,
// }

fn enum_option_1() {
    let some_number = Some(5);
    let some_string = Some("A String");

    let absent_number: Option<i32> = None;
}

// fn enum_option_2() {
//     let x = Some(5);
//     let y = 6;
//     let z = x + y; //error 必须先进行类型转换才能处理
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("test");
            1
        }
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("{:#?}", state);
            25
        }
        _ => 10, // 不需要穷举所有的可能性。
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

// if let
// 只关心一种匹配，

fn if_let_1() {
    let v = Some(0u8);

    match v {
        Some(3) => println!("three"),
        _ => println!("others"),
    }
    // => 完全等价于

    if let Some(3) = v {
        println!("three");
    } else {
        println!("others");
    }
}
