fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    println!();

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // loop 可以返回一个值，这个值就是 break 后面表达式的结果
        }
    };
    println!("The result is {result}");

    println!();
    let mut count = 0;
    'counting_up: loop {
        // loop 标签可以用于 break 或 continue 语句，以指定要操作的循环
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;//
            }
            if count == 2 {
                break 'counting_up; // break 语句可以用于跳出当前循环，也可以指定标签来跳出外层循环
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
