use std::panic;

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("第三个元素是 {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("第三个元素是 {third}"),
        None => println!("没有第三个元素。"),
    }

    println!();

    let mut count: i32 = 0;
    for element in &v {
        count += 1;
        println!("第 {count} 个元素是 {element}");
    }

    // 使用 catch_unwind 捕获 panic
    let result = panic::catch_unwind(|| {
        //let does_not_exist = &v[100]; //引发 panic
        //println!("第100个元素是: {}", does_not_exist);
    });

    match result {
        Ok(_) => println!("成功访问元素"),
        Err(_) => println!("捕获到 panic: 索引越界，第100个元素不存在"),
    }

    println!("程序继续运行...");
    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(value) => println!("第100个元素是: {}", value),
        None => println!("第100个元素不存在,使用 get 方法安全访问,不会引发 panic"),
    }

    println!();
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);



    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // 这样才算"读取"字段
   for cell in &row {
       match cell {
           SpreadsheetCell::Int(value) => println!("Int值是: {}", value),
           SpreadsheetCell::Float(value) => println!("Float值是: {}", value),
           SpreadsheetCell::Text(value) => println!("String 值是: {}", value),
       }
   }

}
