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
}
