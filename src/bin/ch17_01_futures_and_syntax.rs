use trpl::Html;
use helloworld::print_line_separator;

fn demo_1() {
    async fn page_title(url: &str) -> Option<String> {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
    let url = "https://baidu.com";

    // trpl::run 提供异步运行时，驱动 async 块内的 Future 执行
    // .await 只能在 async 函数或 async 块内使用
    trpl::run(async {
        let title = page_title(url).await;
        println!("{:?}", title);
    });
}

fn demo_2(){
    fn page_title(url: &str) -> impl Future<Output = Option<String>> {
        // async move 块：将此块定义为一个异步块，并通过 move 关键字将外部变量（此处为 url）的所有权
        // 转移进来。由于普通函数（非 async fn）不能直接 .await，所以用 async 块来包裹异步逻辑。
        // move 是必要的：url 是一个 &str 引用，如果不 move，异步块的生命周期可能比 url 更长，
        // 导致悬垂引用。move 后，url 的所有权（对引用本身的持有）被转入异步块，编译器才能
        // 安全地将其包装成 Future 返回给调用者。
        async move {
            let text = trpl::get(url).await.text().await;
            Html::parse(&text)
                .select_first("title")
                .map(|title| title.inner_html())
        }
    }
    let url = "https://baidu.com";
    trpl::run(async {
        let title = page_title(url).await;
        println!("{:?}", title);
    });
}

fn main() {
    demo_1();
    print_line_separator();
    demo_2();
}