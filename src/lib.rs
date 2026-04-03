//! Core greeting logic for the helloworld binary.

/// Returns the default greeting string.
pub fn greeting() -> &'static str {
    "Hello, world!"
}

/// Prints the concrete Rust type name of a value with an optional prefix.
pub fn print_type_of<T>(_: &T, prefix: Option<&str>) {
    match prefix {
        Some(p) => println!("{p}: {}", std::any::type_name::<T>()),
        None => println!("{}", std::any::type_name::<T>()),
    }
}

pub fn print_line_separator() {
    println!("{}\n","-".repeat(60));
}


// 处理 catch_unwind 返回的结果
// catch_unwind 返回 Result<T, Box<dyn Any + Send>>
// T 是闭包的返回类型，这里是 ()
// Box<dyn Any + Send> 是 panic 的载荷类型
pub fn process_result(result: Result<(), Box<dyn std::any::Any + Send>>, prefix: Option<&str>) {
    let formatted_prefix = prefix.map_or(String::new(), |p| format!("{} -> ", p));

    match result {
        Ok(_) => println!("{}执行成功", formatted_prefix),
        Err(e) => {
            // 尝试将 panic 信息转换为字符串
            if let Some(s) = e.downcast_ref::<&str>() {
                println!("{}panic 已捕获: {}", formatted_prefix, s);
            } else if let Some(s) = e.downcast_ref::<String>() {
                println!("{}panic 已捕获: {}", formatted_prefix, s);
            } else {
                println!("{}panic 已捕获: 未知错误", formatted_prefix);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_is_expected() {
        assert_eq!(greeting(), "Hello, world!");
    }
}
