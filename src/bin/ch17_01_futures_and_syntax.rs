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
   let t=  page_title(url);


}

fn main() {
    demo_1();
}