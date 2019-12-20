// 使用结构体组织相关联的数据
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 使用结构体
fn test_main1() {
    let user = User {
        active: false,
        sign_in_count: 1,
        username: String::from("hello"),
        email: String::from("world.com"),
    };

    // 改变结构体的值
    let mut user1 = User {
        active: false,
        sign_in_count: 1,
        username: String::from("hello"),
        email: String::from("world.com"),
    };
    user1.email = String::from("xxxxx");

    // 使用结构体更新语法从其他实例创建实例
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    // 简单写法
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1// 简单写法
    };
}

fn build_user1(email: String, username: String) -> User {
    User {
        active: false,
        sign_in_count: 1,
        username: username,
        email: email,
    }
}

// 简单写法
fn build_user2(email: String, username: String) -> User {
    User {
        active: false,
        sign_in_count: 1,
        username,
        email,
    }
}

// 使用没有命名字段的元组结构体来创建不同的类型
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn test_main2() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // 注意 black 和 origin 值的类型不同，因为它们是不同的元组结构体的实例。
    // 定义的每一个结构体有其自己的类型，即使结构体中的字段有着相同的类型。
}

// Rust 为我们提供了很多可以通过 derive 注解来使用的 trait，可以为我们的自定义类型增加实用的行为。
#[derive(Debug)]// 包含打印出调试信息的功能
struct Rectangle {
    width: u32,
    height: u32,
}

fn test_main3() {
    let rec = Rectangle {
        width: 100,
        height: 200,
    };

    println!("area = {}", test_main3_1(&rec));

    // :? 指示符告诉 println! 想要使用叫做 Debug 的输出格式
    // 打印结构体
    println!("rec is {:?}", rec);
    println!("rec is {:#?}", rec);
}

fn test_main3_1(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}

// 给结构体定义方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

// 使用结构体的方法
fn test_main3_2() {
    let rec = Rectangle { width: 100, height: 200 };
    println!("area: {}", rec.area());
}

/*
没有任何字段的类单元结构体
可以定义一个没有任何字段的结构体！它们被称为 类单元结构体（unit-like structs）因为它们类似于 ()，即 unit 类型。
类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。
*/
struct unit_str;

// 类单元结构体
struct str1(String);

// 元组结构体
struct str2(u32, u32);// 元组结构体

// 枚举
enum IPAddrKind {
    IPV4,
    IPV6,
}

enum IPAddr {
    V4(String),
    V6(String),
    VX(u32, f64),
}

/// 给枚举定义方法
impl IPAddr {
    fn test(&self) {}
}

fn test_main4() {
    let ipv4 = IPAddrKind::IPV4;
    let ipv6 = IPAddrKind::IPV6;

    let v4 = IPAddr::V4(String::from("v4"));
    let v6 = IPAddr::V6(String::from("v6"));
    let vx = IPAddr::VX(1, 0.2);
}

fn test_main4_1(ip_type: IPAddrKind) {}

/// Option 枚举
fn test_main5() {
    let some_num = Some(5);
    let some_str = Some("hello");
    // 使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的，
    // 因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型。
    let absent_num: Option<i32> = None;

    let x: Option<i32> = Some(2);
    assert_eq!(x.is_some(), true);

    let x: Option<i32> = None;
    assert_eq!(x.is_some(), false);
}

enum Coin {
    Pennu,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(cion: Coin) -> u8 {
    match cion {// 默认枚举中的每个都必须比配
        Coin::Dime => 1,
        Coin::Pennu => 2,
        Coin::Nickel => 3,
        Coin::Quarter => 4,
    }
}

fn value_in_cents1(cion: Coin) -> u8 {
    match cion {
        Coin::Dime => {
            println!("--dime--");
            1
        }
        Coin::Pennu => 2,
        _ => ()// 匹配所有
    }
}

// 绑定模式值
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Cion1 {
    Dime,
    Quarter(UsState),
}

fn value_in_cents2(coin: Coin1) -> u8 {
    match coin {
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// 匹配Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

/// if let 匹配
fn test_main6(){
    let x = Some(3);
    if let Some(3) = x {
        println!("--3--");
    }
}


fn main() {
    println!("main hello rust\n");
    test_main3_2();
}