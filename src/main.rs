mod test;
mod test1;

mod parser;

use std::fs::File;
use std::io::ErrorKind;

use std::process;

use std::env;
use std::fs;

fn test_main1() {
    // mod test
    test::a::add();
    test::b::add();
    test::a::a::add();

    // mod test1
    test1::c::aaa::add();
}
/*
    vector 允许我们一个挨着一个地储存一系列数量可变的值
    字符串（string）是一个字符的集合。我们之前见过 String 类型
    哈希 map（hash map）允许我们将值与一个特定的键（key）相关联
*/
fn test_main2() {
    // vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("v[2] = {}", &v[1]);


    let v: Vec<i32> = vec![1, 2, 3];
}

fn test_main3() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // 使用 get 方法以索引作为参数来返回一个 Option<&T>
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

/// error
fn test_main4() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    // cannot borrow `v` as mutable because it is also borrowed as immutable
    // 不能这么做的原因是由于 vector 的工作方式：在 vector 的结尾增加新元素时，
    // 在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将
    // 老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。
    // 借用规则阻止程序陷入这种状况。
    // v.push(6);

    println!("The first element is: {}", first);
}

// 遍历vector
fn test_main5() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    // 为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值。
    for i in &mut v {
        *i += 50;
    }
}

fn test_main6() {
    let mut v: Vec<String> = Vec::new();
    let mut s = String::from("hello");
    v.push(s);
    // error
    // println!("s = {}",s);
}

/// map
/// 像 vector 一样，哈希 map 将它们的数据储存在堆上，这个 HashMap 的键类型是 String 而值类型是 i32。
/// 类似于 vector，哈希 map 是同质的：所有的键必须是相同类型，值也必须都是相同类型。
use std::collections::HashMap;

fn test_main7() {
    let mut msp = HashMap::new();
    msp.insert(String::from("hello"), 10);
    msp.insert(String::from("world"), 20);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // 访问哈希 map 中的值
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
}

// panic
fn test_main8() {
    let v = vec![1, 2, 3];
    let i = v[99];
}

fn test_main9() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => { file }
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

/// 失败时 panic 的简写：unwrap 和 expect
// match 能够胜任它的工作，不过它可能有点冗长并且不总是能很好的表明其意图。Result<T, E> 类型定义了很多辅助方法来处理各种情况。其中之一叫做 unwrap。
// 如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。如果 Result 是成员 Err，unwrap 会为我们调用 panic!
fn test_main10() {
    // 如果调用这段代码时不存在 hello.txt 文件， unwrap 调用 panic! 时提供的错误信息：
    let f = File::open("hello.txt").unwrap();

    // 使用 expect 而不是 unwrap 并提供一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    // expect 与 unwrap 的使用方式一样：返回文件句柄或调用 panic! 宏。expect 用来调用 panic! 的错误信息将会作为参数传递给 expect ，而不像unwrap 那样使用默认的 panic! 信息。它看起来像这样：
}

/// trait 类似java中的abstract class
pub trait Summary {
    // 类似java的抽象方法 只有声明 没有实现
    fn summary(&self) -> String;

    // 提供一个默认实现的方法
    fn summary_1(&self) -> String {
        String::from("summary_1 ...")
    }
}

struct News {
    text: String,
}

/// News实现trait Summery
/// 实现 trait 时需要注意的一个限制是，
/// 只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait。
impl Summary for News {
    // 实现summary方法
    fn summary(&self) -> String {
        String::from("hello news")
    }
}

fn test_main11() {
    let v = News { text: String::from("hello") };
    println!("news text: {}", v.summary());
    println!("news text: {}", v.summary_1());
}

fn test_main12(item: impl Summary/* item是实现了Summmary的参数 */) {
    println!("item.summary = {}", item.summary());
}

fn test_main13() {
    let x = 22;
    let y = x;
    println!("x = {}", x);
    println!("y = {}", y);

    let s = String::from("hello");
    let s1 = s;
    // error borrow of moved value
    // println!("s = {}",s);
    println!("s1 = {}", s1);

    let s2 = s1.clone();
    println!("s2 = {}", s2);

    test_main13_1(s2);
    // 将值传递给函数在语义上与给变量赋值相似。
    // 向函数传递值可能会移动或者复制
    // borrow of moved value:
    // println!("s2 = {}", s2);

    /*
        变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。

        引用& -> 借用，不会获取所有权
        赋值 —> move,获取原来变量的所有权，导致原来的变量无效

        引用的作用域从声明的地方开始一直持续到最后一次使用为止

        生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。一旦他们形成了某种关联，Rust 就有了足够的信息来允许内存安全的操作并阻止会产生悬垂指针亦或是违反内存安全的行为。

    */
}

fn test_main13_1(s: String) {
    println!("s = {}", s);
}

// 结构体定义中的生命周期注解
struct ImportantExcerpt<'a> {
    part: &'a str,
}

struct AA<'A> {
    name: &'A str,
}

// 读取命令行参数
/*
    注意 std::env::args 在其任何参数包含无效 Unicode 字符时会 panic。
    如果你需要接受包含无效 Unicode 字符的参数，使用 std::env::args_os 代替。
    这个函数返回 OsString 值而不是 String 值。这里出于简单考虑使用了 std::env::args，
    因为 OsString 值每个平台都不一样而且比 String 值处理起来更为复杂。
*/
fn test_main14() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);// ":?" 调试格式
    let len = args.len();
    if len >= 3 {
        let query = &args[1];
        let fileName = &args[2];
        println!("query = {}", query);
        println!("fileName = {}", fileName);
    } else {
        println!("please input args");
    }
}

//  读取文件内容
fn test_main15() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 {
        let query = &args[1];
        let fileName = &args[2];

        let fileContent = std::fs::read_to_string(fileName).expect("faild to read");

        println!("fileContent: {}", fileContent);
    } else {
        println!("please input args ...")
    }

//    let fileContent = std::fs::read_to_string(fileName);
//    match fileContent {
//        Err(e)=>{},
//
//    }
}

fn test_main16() {
    let args: Vec<String> = std::env::args().collect();
    /*
        unwrap_or_else，它定义于标准库的 Result<T, E> 上。
        使用 unwrap_or_else 可以进行一些自定义的非 panic! 的错误处理。
        当 Result 是 Ok 时，这个方法的行为类似于 unwrap：它返回 Ok 内部封装的值。
        然而，当其值是 Err 时，该方法会调用一个 闭包（closure），也就是一个我们定义的作为参数传递给 unwrap_or_else 的匿名函数
    */
    let config = parser::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
}

use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

/// 闭包
/// Rust 的 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数。可以在一个地方创建闭包，然后在不同的上下文中执行闭包运算。不同于函数，闭包允许捕获调用者作用域中的值
/*

let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};

闭包定义是 expensive_closure 赋值的 = 之后的部分。
闭包的定义以一对竖线（|）开始，在竖线中指定闭包的参数；
之所以选择这个语法是因为它与 Smalltalk 和 Ruby 的闭包定义类似。
这个闭包有一个参数 num；如果有多于一个参数，可以使用逗号分隔，
比如 |param1, param2|。

参数之后是存放闭包体的大括号 —— 如果闭包体只有一行则大括号是可以省略的。
大括号之后闭包的结尾，需要用于 let 语句的分号。
因为闭包体的最后一行没有分号（正如函数体一样），所以闭包体（num）最后一行的返回值作为调用闭包时的返回值 。

*/
fn test_main17() {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let expensive_closure = |num: i32/*参数类型*/| -> i32/*闭包返回值类型*/ {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // error
    // 第一次使用 String 值调用 example_closure 时，
    // 编译器推断 x 和此闭包返回值的类型为 String。
    // 接着这些类型被锁定进闭包 example_closure 中，如果尝试对同一闭包使用不同类型则会得到类型错误。
//    let example_closure = |x| x;
//    let s = example_closure(String::from("hello"));
//    let n = example_closure(5);

    // Fn 系列 trait 由标准库提供。所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个。
}
/*
定义一个 Cacher 结构体来在 calculation 中存放闭包并在 value 中存放 Option 值
结构体 Cacher 有一个泛型 T 的字段 calculation。T 的 trait bound
指定了 T 是一个使用 Fn 的闭包。任何我们希望储存到 Cacher
实例的 calculation 字段的闭包必须有一个 u32 参数（由 Fn 之后的括号的内容指定）
并必须返回一个 u32（由 -> 之后的内容）

闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：
获取所有权，可变借用和不可变借用。
这三种捕获值的方式被编码为如下三个 Fn trait：
1、FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。
为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
2、FnMut 获取可变的借用值所以可以改变其环境
3、Fn 从其环境获取不可变的借用值


*/
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

fn main() {
    println!("**** hello rust ****");

    test_main16();
}