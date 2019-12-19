// 从标准库引入输入输出
// use std::io;

// 从标准库引入了一个叫做 std::cmp::Ordering 的类型。同 Result 一样， Ordering 也是一个枚举，不过它的成员是 Less、Greater 和 Equal。这是比较两个值时可能出现的三种结果。
use std::cmp::Ordering;

use rand::Rng;// 使用从rust开源仓库中下载的第三方库 生成随机数

/*
    fn 声明函数
    println! 在屏幕上打印字符串的宏
*/
fn test1() {
    println!("Guess the number!");
    println!("please input you guess");

    // "::" 表明new是String的关联函数
    let mut guess = String::new();// 返回一个可变字符串
    std::io::stdin()
        .read_line(&mut guess)// read_line返回io::Result
        .expect("faild to read line");

    println!("you guess: {}", guess);// "{}"占位符

    /*
        use std::io 导入
        std::io 标准输入输出

        Result 类型是 枚举（enumerations）
        Result 的成员是 Ok 和 Err，Ok 成员表示操作成功，内部包含成功时产生的值。Err 成员则意味着操作失败，并且包含失败的前因后果
    */
}

/// 生成随机数
/// use rand::Rng。Rng 是一个 trait，
/// 它定义了随机数生成器应实现的方法，想使用这些方法的话，此 trait 必须在作用域中
fn test2() {
    // 生成随机数
    let secret_number = rand::thread_rng().gen_range(1, 5);// gen_range包头不包尾巴
    println!("the random secret number is: {}", secret_number);
}

fn test3() {
    println!("input number ....");
    let mut num_str = String::new();
    std::io::stdin().read_line(&mut num_str).expect("faild to read");
    println!("you put the number: {}", num_str);
    // 字符串转换为数字
    // 字符串的 parse 方法 将字符串解析成数字。
    // 因为这个方法可以解析多种数字类型，因此需要告诉 Rust 具体的数字类型
    let num: u32 = num_str.trim().parse().expect("please number type");

    let random_num = rand::thread_rng().gen_range(1, 100);

    println!("random_num = {}", random_num);

    // 比较
    match num.cmp(&random_num) {
        Ordering::Less => {
            println!("too small");
        }
        Ordering::Greater => {
            println!("Too big");
        }
        Ordering::Equal => {
            println!("equal!");
        }
    }
}

fn test4() {
    println!("input ....");
    // 循环
    loop {
        let mut str = String::new();
        let str1 = "hello";
        match str.cmp(&String::from(str1)) {
            Ordering::Equal => {}
            Ordering::Greater => {}
            Ordering::Less => {}
        };
    }
}

fn test5() {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num);
}

fn test6() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0, 100);

    println!("secret_number: {}", secret_number);
    loop {
        println!("please input you guess!");
        let mut guess = String::new();

        std::io::stdin().read_line(&mut guess).expect("faild to readline");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("you input the guess {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("too small");
            }
            Ordering::Greater => {
                println!("too big");
            }
            Ordering::Equal => {
                println!("Equal");
            }
        };
    }
}

fn test7(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        std::io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn test8(){
    // 常量
    const COUNT:u32 = 100;

    let mut spaces = "    ";
    let mut spaces = spaces.len();
}

fn test9(){
    // 标量（scalar）和复合（compound）
    // Rust有四种基本的标量类型：整型、浮点型、布尔类型和字符类型
    // 数字类型默认是 i32：它通常是最快的，甚至在 64 位系统上也是。
    // isize 或 usize 主要作为某些集合的索引。
    // Rust 的浮点数类型是 f32 和 f64，分别占 32 位和 64 位。默认类型是 f64，
    // 因为在现代 CPU 中，它与 f32 速度几乎一样，不过精度更高。
    let i:i32 = 1;
    let i1:u32 = 1;
    let f:f32 = 0.1;
    let f1:f64 = 0.1;
    let b = true;
    let b1:bool = false;

    // Rust 的 char 类型的大小为四个字节(four bytes)，
    // 并代表了一个 Unicode 标量值（Unicode Scalar Value）
    let ch:char = 'a';
    let ch1 = 'a';

    // 复合（compound）
    // Rust 有两个原生的复合类型：元组（tuple）和数组（array）。


}

fn main() {
    println!("hello rust\n");
    test7();
}
