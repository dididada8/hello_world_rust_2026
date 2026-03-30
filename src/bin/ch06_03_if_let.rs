fn demo_1(){
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        // `_` 是通配符模式：匹配所有未被前面分支匹配到的情况（这里就是 `None`）。
        // `=>` 读作“匹配后执行”，左边是模式，右边是该分支表达式。
        // `()` 是单元值（unit），表示“什么都不做”；因此这个分支仅用于占位兜底。
        _ => (),
    }
}

fn main(){
    demo_1();
}
