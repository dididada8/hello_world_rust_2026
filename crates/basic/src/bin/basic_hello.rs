use helloworld::greeting;
use helloworld::print_type_of;

fn main() {
    println!("{}", greeting());
    let x = 5;
    let y: i32 = 10;
    println!("{} + {} = {}", x, y, x + y);

    // x = 6;  // 错误！不能重新赋值

    let mut counter = 0;
    counter += 1;
    counter += 2;
    println!("counter = {}", counter);

    const MAX_POINTS: u32 = 100_000;
    const PI: f64 = 3.1415926535897932384626433832795;
    println!("The value of MAX_POINTS is {}", MAX_POINTS);
    println!("The value of PI is {}", PI);

    //变量遮蔽（Shadowing）
    let x = 6;
    let x = x + 1;
    println!("x = {}", x);
    let x = x * 2;
    println!("x = {}", x);
    println!();

    // 遮蔽可以改变类型
    let spaces = "   ";
    print_type_of(&spaces, Some("spaces(&str)"));
    let spaces = spaces.len(); // &str -> usize
    print_type_of(&spaces, Some(&format!("spaces({spaces})")));

    println!("current spaces = {}", spaces);
    println!();

    //基本数据类型
    let a: i32 = 42;
    let b: u64 = 100;
    let c: isize = -50;

    println!("a = {}, b = {}, c = {}", a, b, c);

    println!();

    // 数字字面量
    let decimal = 99_220; // 99220
    let hex = 0xff; // 255
    let octal = 0o77; // 63
    let binary = 0b1111_0000; // 240
    let byte = b'A'; // 65 (u8)
    println!(
        "decimal = {}, hex = {}, octal = {}, binary = {}, byte = {}",
        decimal, hex, octal, binary, byte
    );

    println!();

    //布尔类型
    let t = true;
    let f: bool = false;
    println!("t = {}, f = {}", t, f);

    //字符类型
    let c = 'z';
    let heart: char = '❤';
    let japanese = 'あ';

    println!("c = {}, heart = {},japanese={}", c, heart, japanese);

    // char 是 4 字节的 Unicode 标量值
    println!("size of char: {} bytes", std::mem::size_of::<char>());

    println!();

    //复合类型-元组（Tuple）
    let tuple: (i32, f64, char) = (50, 3.14, 'A');

    // 解构
    let (x, y, z) = tuple;
    println!("x = {}, y = {}, z = {}", x, y, z);

    // 索引访问
    let first = tuple.0;
    let second = tuple.1;
    println!("first = {}, second = {},third={}", first, second, tuple.2);

    // 空元组
    let unit: () = ();
    println!("unit = {:?}", unit);

    println!();

    //复合类型-数组（Array）
    println!("复合类型-数组（Array）");
    // 固定长度数组
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // 简写：重复元素
    let zeros = [0; 5]; // [0, 0, 0, 0, 0]
    println!("{:?}", zeros);

    // 访问元素
    let first = arr[0];
    let second = arr[1];
    println!("first = {}, second = {}", first, second);

    // 数组长度
    let len = arr.len();

    println!("数组长度：{}", len);


    println!();

    // 显式转换（不会溢出检查）
    let a: u8 = 255;
    let b: u32 = a as u32;

    // 可能丢失精度
    let c: f64 = 3.9;
    let d: u32 = c as u32;  // 3 (截断小数)

    println!("b = {}, d = {}", b, d);


}
