use helloworld::print_line_separator;
use trpl::Html;

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

fn demo_2() {
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

    // 1#
    trpl::run(async {
        let title = page_title(url).await;
        println!("{:?}", title);
    });

    // 2#
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let title = page_title(url).await;
        println!("{:?}", title);
    });
    println!("{}", url);
}

fn demo_3() {
    // 与 demo_1 的 page_title 相比，返回值从 Option<String> 改为 (Option<String>, &str)，
    // 额外把 url 一并返回。原因：trpl::select 只告诉我们"哪个 Future 先完成"，
    // 但不告诉我们 url 是哪个，所以需要在 Future 内部把 url 一起打包返回，
    // 才能在外部知道是哪个网站先响应。
    async fn page_title(url: &str) -> (Option<String>, &str) {
        let text = trpl::get(url).await.text().await;

        let title = Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html());
        // 将 title 和 url 作为元组一起返回
        (title, url)
    }

    let url_1 = "https://baidu.com";
    let url_2 = "https://google.com";

    // trpl::block_on 与 trpl::run 作用相同，都是在同步上下文中启动一个异步运行时来驱动 Future
    trpl::block_on(async {
        // 此处只是"构造 Future"，并未开始执行。
        // Rust 的 Future 是惰性的（lazy）：只有被 .await 或交给运行时驱动时才真正运行。
        let title_fut_1 = page_title(url_1);
        let title_fut_2 = page_title(url_2);

        // trpl::select 同时轮询（poll）两个 Future，哪个先完成就立刻返回，另一个被丢弃。
        // 这实现了"竞速（race）"语义：谁响应快用谁的结果。
        // 返回类型是 trpl::Either<L, R>：
        //   - Either::Left(val)  表示 title_fut_1（第一个参数）先完成，val 是其返回值
        //   - Either::Right(val) 表示 title_fut_2（第二个参数）先完成，val 是其返回值
        // 注意：两个分支的解构顺序都是 (title, url)，对应 page_title 返回的元组，
        //       然后将其映射为 (url, title) 统一变量名，方便后续使用。
        let (url, title) = match trpl::select(title_fut_1, title_fut_2).await {
            trpl::Either::Left((title, url)) => (url, title),
            trpl::Either::Right((title, url)) => (url, title),
        };

        match title {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    });
}

fn main() {
    demo_1();
    print_line_separator();
    demo_2();
    print_line_separator();
    demo_3();
}
