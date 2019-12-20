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

fn test7() {
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

fn test8() {
    // 常量
    const COUNT: u32 = 100;

    let mut spaces = "    ";
    let mut spaces = spaces.len();
}

/// 变量
fn test9() {
    // 标量（scalar）和复合（compound）
    // Rust有四种基本的标量类型：整型、浮点型、布尔类型和字符类型
    // 数字类型默认是 i32：它通常是最快的，甚至在 64 位系统上也是。
    // isize 或 usize 主要作为某些集合的索引。
    // Rust 的浮点数类型是 f32 和 f64，分别占 32 位和 64 位。默认类型是 f64，
    // 因为在现代 CPU 中，它与 f32 速度几乎一样，不过精度更高。
    let i: i32 = 1;
    let i1: u32 = 1;
    let f: f32 = 0.1;
    let f1: f64 = 0.1;
    let b = true;
    let b1: bool = false;

    // Rust 的 char 类型的大小为四个字节(four bytes)，
    // 并代表了一个 Unicode 标量值（Unicode Scalar Value）
    let ch: char = 'a';
    let ch1 = 'a';

    // 复合（compound）
    // Rust 有两个原生的复合类型：元组（tuple）和数组（array）。
    // 元组类型
    // 元组是一个将多个其他类型的值组合进一个复合类型的主要方式。
    // 元组长度固定：一旦声明，其长度不会增大或缩小。
    let tup: (i32, f64, u8) = (1, 0.1, 1);
    // 使用模式匹配（pattern matching）来解构（destructure）元组值
    let tup1 = (100, 8.9, false);
    let (x, y, z) = tup1;
    println!("x = {},y = {},z = {}", x, y, z);
    // 使用模式匹配解构外，也可以使用点号（.）后跟值的索引来直接访问它们
    let tup2: (i32, i32, i32) = (1, 2, 3);
    println!("tup2.0 = {},tup2.1 = {},tup2.2 = {}", tup2.0, tup2.1, tup2.2);

    // 数组 // vector是可变的数组
    // 与元组不同，数组中的每个元素的类型必须相同。
    // Rust 中的数组与一些其他语言中的数组不同，
    // 因为 Rust 中的数组是固定长度的：一旦声明，它们的长度不能增长或缩小。
    let arr: [i32; 4] = [1, 2, 3, 4];
    let arr1 = ["hello", "world"];
    let arr2 = [3; 5];// 数组的每个元素都是3 长度是5
    println!("arr[0] = {}", arr[0]);
}

/// 函数也可以被定义为拥有 参数（parameters），参数是特殊变量，是函数签名的一部分。
/// 当函数拥有参数（形参）时，可以为这些参数提供具体的值（实参）。
/// 技术上讲，这些具体值被称为参数（arguments）
fn test10_1(i: i32) {
    println!("i = {}", i);
}

fn test10() {
    test10_1(20);
}

/// 包含语句和表达式的函数体
/// 语句（Statements）是执行一些操作但不返回值的指令。表达式（Expressions）计算并产生一个值
fn test11() {
    // 语句 函数定义也是语句
    let x = 3;

    let x = x + 1;

    let y = {
        let x = 3;
        x + 1// 注意没有分号，加上分号之后就变成了语句，rust中语句是没有返回值的，
        // 作为一个值绑定到y
    };
}

fn test12() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

/// 具有返回值的函数
/// 在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。
/// 使用 return 关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式
fn test13() -> i32 {
    5
}

// 控制流
fn test14() {
    // if
    let num = 100;
    if num < 20 {
        println!("num < 50");
    } else if num < 30 {
        println!("num < 25");
    } else {
        println!("num > 30");
    }

    // 在 let 语句中使用 if
    let con = true;
    let num = if con {
        100
    } else {
        200
    };
    println!("num = {}", num);
}

// 循环 Rust 为此提供了多种 循环（loops）
// Rust 有三种循环：loop、while 和 for
fn test15() {
    // loop
    if true {
        let mut i = 0;
        loop {
            println!("hello {}", i);
            i = i + 1;
            if i > 5 {
                break;
            }
        }
    }
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // while
    let mut num = 3;
    while num != 0 {
        num = num - 1;
    }
    println!("num = {}", num);

    // for
    let arr = [1, 2, 3, 4, 5];
    for ele in arr.iter() {
        println!("ele = {}", ele);
    }
    println!("/////////////////////");
    for ele in (1..3).rev() {// (1..3)包头不包尾
        println!("ele = {}", ele);
    }
}

// 所有权
// 通过所有权系统管理内存，
// 编译器在编译时会根据一系列的规则进行检查。在运行时，所有权系统的任何功能都不会减慢程序。
/*
栈中的所有数据都必须占用已知且固定的大小。在编译时大小未知或大小可能变化的数据，
要改为存储在堆上。堆是缺乏组织的：当向堆放入数据时，你要请求一定大小的空间。
操作系统在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该位置地址的
指针（pointer）。这个过程称作 在堆上分配内存（allocating on the heap），
有时简称为 “分配”（allocating）。将数据推入栈中并不被认为是分配。因为指针的大小
是已知并且固定的，你可以将指针存储在栈上，不过当需要实际数据时，必须访问指针。
*/
// 跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，
// 以及清理堆上不再使用的数据确保不会耗尽空间，这些问题正是所有权系统要处理的。

/*
    所有权的规则：
        Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
        值有且只有一个所有者。
        当所有者（变量）离开作用域，这个值将被丢弃。
*/
fn test16() {
    let mut s = String::from("hello");
    // 字符串 加
    s.push_str(" world");
    println!("s = {}", s);
}

// 内存在拥有它的变量离开作用域后就被自动释放
fn test17() {
    {
        let s = String::from("hello");
        println!("s = {}", s);
        // 当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 drop
        // 当变量离开作用域后，Rust 自动调用 drop 函数并清理变量的堆内存
        drop(s);
    }

    // error: not found in this scope
    // 此作用域已结束，s 不再有效
    // println!("s = {}", s);
}


fn test18() {
    // 变量与数据交互的方式（一）：移动

    // “将 5 绑定到 x；接着生成一个值 x 的拷贝并绑定到 y”。
    // 现在有了两个变量，x 和 y，都等于 5。这也正是事实上发生了的，
    // 因为整数是有已知固定大小的简单值，所以这两个 5 被放入了栈中。
    let x = 5;
    let y = x;

    // String 由三部分组成：一个指向存放字符串内容内存的指针，一个长度，和一个容量。这一组数据存储在栈上
    // 其中的内容是堆上
    // 长度表示 String 的内容当前使用了多少字节的内存。
    // 容量是 String 从操作系统总共获取了多少字节的内存
    // 当我们将 s1 赋值给 s2，String 的数据被复制了，这意味着我们从栈上拷贝了它的指针、长度和容量。我们并没有复制指针指向的堆上数据
    let s1 = String::from("hello");
    let s2 = s1;
    // error  Rust使s1变量无效了，rust这个操作被称为 移动（move）
    // println!("s1 = {}",s1);
    // 注意 Rust 永远也不会自动创建数据的 “深拷贝”

    // 变量与数据交互的方式：克隆
    // 如果确实 需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的通用函数。
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {},s4 = {}", s3, s4);

    // 只在栈上的数据：拷贝
    // 像整型这样的在编译时已知大小的类型被整个存储在栈上，
    // 所以拷贝其实际的值是快速的。这意味着没有理由在创建变量 y 后使 x 无效
    // Rust 有一个叫做 Copy trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上
    //
    // 如果一个类型拥有 Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用。Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait。
    // 如果我们对其值离开作用域时需要特殊处理的类型使用 Copy 注解，将会出现一个编译时错误
    let x = 5;
    let y = x;
    println!("x = {},y = {}", x, y);
    /*
        作为一个通用的规则，任何简单标量值的组合可以是 Copy 的，不需要分配内存或某种形式资源的类型是 Copy 的。如下是一些 Copy 的类型：

        所有整数类型，比如 u32。
        布尔类型，bool，它的值是 true 和 false。
        所有浮点数类型，比如 f64。
        字符类型，char。
        元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。
    */
}

// 所有权与函数
// 将值传递给函数在语义上与给变量赋值相似。向函数传递值可能会移动或者复制，就像赋值语句一样
fn test19() {
    let s = String::from("hello");
    println!("s = {}", s);
    test19_1(s);
    // error s被move 无效了
    // println!("s = {}",s);
    let i = 1;
    test19_2(i);
    //  i应该移动函数里，
    //  但 i32 是 Copy 的，所以在后面可继续使用 i
    println!("i = {}", i);
}

fn test19_1(s: String) {}

fn test19_2(i: i32) {}

// 返回值与作用域
fn test20() {
    // gives_ownership 将返回值移给 s1
    let s1 = gives_ownership();

    // s2 进入作用域
    let s2 = String::from("hello");

    // s2 被移动到takes_and_gives_back 中,它也将返回值移给 s3
    let s3 = takes_and_gives_back(s2);

    // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
    // 所以什么也不会发生。s1 移出作用域并被丢弃
}

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}
/*
    变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。
    当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。
*/

/*
在每一个函数中都获取所有权并接着返回所有权有些啰嗦。如果想要函数使用一个值但不获取所有权该怎么办呢？
如果还要接着使用它的话，每次都传进去再返回来太麻烦，
除此之外，我们也可能想返回函数体中产生的一些数据。我们可以使用元组来返回多个值
*/

fn test21() {
    let s1 = String::from("hello");
    let (s2, len) = test21_1(s1);
    println!("s2 = {},len = {}", s2, len);

    // 传递引用
    // &s2 语法让我们创建一个 指向 值 s2 的引用，但是并不拥有它。
    // 因为并不拥有这个值，当引用离开作用域时其指向的值也不会被丢弃。
    let n = test21_2(&s2);
    println!("s2 = {},n = {}", s2, n);

    // 传递 可变引用
    let mut s3 = String::from("hello ");
    let n = test21_3(&mut s3);

    // 可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s;
    // error 数据有且只能有一个可变引用
    // 这个限制的好处是 Rust 可以在编译时就避免数据竞争
    // println!("{}, {}", r1, r2);

    // 不能在拥有不可变引用的同时拥有可变引用
    let mut s = String::from("hello");
//    let r1 = &s; // 没问题
//    let r2 = &s; // 没问题
//    let r3 = &mut s; // 大问题
//    // error
//    println!("{}, {}, and {}", r1, r2, r3);

    // 注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用
    let r3 = &mut s; // 没问题
    println!("{}", r3);
}

// 麻烦 可以使用引用处理
fn test21_1(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

// 使用引用处理
// & 符号就是 引用，它允许你使用值但不获取其所有权
// 与使用 & 引用相反的操作是 解引用（dereferencing），它使用解引用运算符，*
fn test21_2(s: &String) -> usize {// 将获取引用作为函数参数称为 借用（borrowing)
    let len = s.len();
    // error （默认）不允许修改引用的值。
    // s.push_str("");
    len
    // s离开了作用域。但因为它并不拥有引用值的所有权，所以什么也不会发生
    // 当引用离开作用域后并不丢弃它指向的数据，因为我们没有所有权
}

// 使用可变引用
fn test21_3(s: &mut String) -> usize {
    s.push_str("world");
    let len = s.len();
    len
}

// 悬垂引用（Dangling References）
// error[E0106]: missing lifetime specifier
//fn test22(){
//    let reference_to_nothing = dangle();
//}
//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s // 返回字符串 s 的引用
// 因为 s 是在 dangle 函数内创建的，当 dangle 的代码执行完毕后，s 将被释放。
// 不过我们尝试返回它的引用。这意味着这个引用会指向一个无效的 String，Rust 不会允许这么做。
//} // 这里 s 离开作用域并被丢弃。其内存被释放。 危险！

//引用的规则
//在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
//引用必须总是有效的。

// slice
// slice是没有所有权的数据类型，slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。
fn test22(){
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    /*
    使用一个由中括号中的 [starting_index..ending_index] 指定的 range 创建一个 slice，
    其中 starting_index 是 slice 的第一个位置，ending_index 则是 slice 最后一个位置的后一个值。
    在其内部，slice 的数据结构存储了 slice 的开始位置和长度，
    长度对应于 ending_index 减去 starting_index 的值。
    所以对于 let world = &s[6..11]; 的情况，world 将是一个包含指向 s 第 7 个字节（从 1 开始）的指针和长度值 5 的 slice。
    */

    // 对于 Rust 的 .. range 语法，如果想要从第一个索引（0）开始，可以不写两个点号之前的值
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];
    // 如果 slice 包含 String 的最后一个字节，也可以舍弃尾部的数字
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    // 可以同时舍弃这两个值来获取整个字符串的 slice
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];

    // 注意：字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内，
    // 如果尝试从一个多字节字符的中间位置创建字符串 slice，则程序将会因错误而退出。

    let s = "hello world";
}
fn main1() {
    println!("hello rust\n");
    test21();
}
