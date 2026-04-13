use helloworld::print_line_separator;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

fn demo_1() {
    trpl::block_on(async {
        // 演示目的：
        // - 不同的 `async { ... }` 块会生成不同的匿名 Future 类型。
        // - `Vec<T>` 要求所有元素都是同一类型，所以不能直接收集多个不同 async 块。
        // - 这里用 trait object 统一类型后，再交给 `join_all` 一次性并发等待。
        //
        // `Box::pin(async { ... })` 的作用：
        // - `Box`：把具体 Future 放到堆上，便于做 `dyn Future` 的类型擦除。
        // - `Pin`：保证该 Future 不再被移动，满足 `Future::poll` 对固定地址的约束。
        // - 最终统一为 `Pin<Box<dyn Future<Output = ()>>>`，可放入同一个 `Vec`。
        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![
            Box::pin(async {
                println!("hello");
            }),
            Box::pin(async {
                println!("world");
            }),
            Box::pin(async {
                println!("hello again");
            }),
        ];
        trpl::join_all(futures).await;
    });
}

fn demo_2() {
    // demo_2: 插件系统/服务启动阶段的并发初始化
    //
    // 生产场景：
    // - 一个服务启动时通常要初始化多个外部依赖（数据库连接池、缓存、消息队列、配置中心）。
    // - 这些初始化任务来自不同模块，通常是不同的 async 函数/async 块，类型彼此不同。
    // - 但我们希望把它们放进同一个列表统一调度，并且并发执行提升启动速度。
    //
    // 这里定义统一任务类型：
    // - Output 统一成 Result<&'static str, &'static str>，方便汇总每个任务结果。
    // - + Send 是生产中常见约束（任务可能被多线程 runtime 调度）。
    type InitTask = Pin<Box<dyn Future<Output = Result<&'static str, &'static str>> + Send>>;

    trpl::block_on(async {
        let tasks: Vec<InitTask> = vec![
            Box::pin(async { Ok("db pool ready") }),
            Box::pin(async { Ok("redis ready") }),
            Box::pin(async { Ok("kafka producer ready") }),
        ];

        // 一次性并发等待全部初始化任务完成，得到每个任务的独立结果。
        let results = trpl::join_all(tasks).await;
        for result in results {
            match result {
                Ok(msg) => println!("[init ok] {msg}"),
                Err(err) => println!("[init err] {err}"),
            }
        }
    });
}

fn demo_3() {
    // demo_3: 动态路由/命令分发
    //
    // 生产场景：
    // - 网关、机器人、RPC 命令系统常需要 "字符串路由 -> 处理函数" 的动态注册。
    // - 每个处理函数内部逻辑不同（数据库查询、缓存读取、权限校验），Future 的具体类型也不同。
    // - 要放到同一个 HashMap 中，必须统一返回类型。
    //
    // 这个 Handler 定义把“异构 async 处理器”统一成同一个可调用签名：
    // - 入参：String（命令参数）
    // - 返回：Pin<Box<dyn Future<Output = String> + Send>>
    type Handler =
        Box<dyn Fn(String) -> Pin<Box<dyn Future<Output = String> + Send>> + Send + Sync>;

    let mut routes: HashMap<&'static str, Handler> = HashMap::new();

    routes.insert("ping", Box::new(|_arg| Box::pin(async { "pong".to_string() })));
    routes.insert(
        "hello",
        Box::new(|name| Box::pin(async move { format!("hello, {name}") })),
    );

    trpl::block_on(async {
        if let Some(handler) = routes.get("ping") {
            let output = handler("ignored".to_string()).await;
            println!("[route ping] {output}");
        }
        if let Some(handler) = routes.get("hello") {
            let output = handler("production-user".to_string()).await;
            println!("[route hello] {output}");
        }
    });
}

trait Job: Send + Sync {
    // demo_4 里故意不用 async fn in trait（在很多项目里仍会考虑对象安全与稳定性问题）。
    // 改用返回 Pin<Box<dyn Future>> 的写法，让 trait object（dyn Job）可直接装进集合。
    fn name(&self) -> &'static str;
    fn run(&self) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + '_>>;
}

struct EmailJob;
struct CleanupJob;

impl Job for EmailJob {
    fn name(&self) -> &'static str {
        "email-job"
    }

    fn run(&self) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + '_>> {
        Box::pin(async move {
            println!("sending email...");
            Ok(())
        })
    }
}

impl Job for CleanupJob {
    fn name(&self) -> &'static str {
        "cleanup-job"
    }

    fn run(&self) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + '_>> {
        Box::pin(async move {
            println!("cleaning temp files...");
            Ok(())
        })
    }
}

fn demo_4() {
    // demo_4: 任务调度器（trait object + async）
    //
    // 生产场景：
    // - 调度器从配置加载一批任务，任务类型动态决定（邮件、清理、同步等）。
    // - 希望统一存储为 Vec<Box<dyn Job>>，再按统一接口执行。
    // - 每个任务 run() 是异步逻辑，具体 Future 类型不同，必须做类型擦除。
    let jobs: Vec<Box<dyn Job>> = vec![Box::new(EmailJob), Box::new(CleanupJob)];

    trpl::block_on(async {
        // 这里演示“统一接口 + 并发执行”：
        // - iter() 拿到每个 dyn Job
        // - run() 返回的都是同一种外壳类型 Pin<Box<dyn Future<...>>>
        // - join_all 可以直接并发等待
        let names: Vec<&'static str> = jobs.iter().map(|job| job.name()).collect();
        let futures: Vec<_> = jobs.iter().map(|job| job.run()).collect();
        let results = trpl::join_all(futures).await;

        for (name, result) in names.into_iter().zip(results) {
            match result {
                Ok(()) => println!("[job ok] {name}"),
                Err(err) => println!("[job err] {name}: {err}"),
            }
        }
    });
}

#[derive(Debug)]
struct Request {
    user_id: u64,
    authorized: bool,
    tags: Vec<&'static str>,
}

fn demo_5() {
    // demo_5: 中间件链（鉴权、审计、打标等）
    //
    // 生产场景：
    // - HTTP/RPC 服务经常有一条“请求处理链”：鉴权 -> 限流 -> 审计 -> 业务逻辑。
    // - 每个中间件都可能是异步操作，且实现差异很大（查缓存、查数据库、写日志）。
    // - 为了可配置和可插拔，常会把中间件放进一个 Vec 动态执行。
    //
    // 统一中间件签名：
    // - 输入/输出都用 Request，便于链式传递与逐步增强上下文。
    // - 返回 Pin<Box<dyn Future<Output = Request> + Send>>，统一异构 async 实现。
    type Middleware =
        Box<dyn Fn(Request) -> Pin<Box<dyn Future<Output = Request> + Send>> + Send + Sync>;

    let middlewares: Vec<Middleware> = vec![
        // 鉴权中间件：通常会调用外部认证服务；这里简化为写入标记。
        Box::new(|mut req| {
            Box::pin(async move {
                req.authorized = true;
                req.tags.push("auth_checked");
                req
            })
        }),
        // 审计中间件：生产里会写审计日志；这里打印并打标。
        Box::new(|mut req| {
            Box::pin(async move {
                println!("[audit] user_id={}", req.user_id);
                req.tags.push("audited");
                req
            })
        }),
        // 业务前预处理：例如请求规范化、灰度信息注入等。
        Box::new(|mut req| {
            Box::pin(async move {
                req.tags.push("normalized");
                req
            })
        }),
    ];

    trpl::block_on(async {
        let mut req = Request {
            user_id: 42,
            authorized: false,
            tags: vec![],
        };

        // 顺序执行中间件链：上一个中间件的输出，作为下一个的输入。
        for middleware in &middlewares {
            req = middleware(req).await;
        }

        println!("[pipeline done] {:?}", req);
    });
}

fn main() {
    println!("===demo 1====");
    demo_1();
    print_line_separator();

    println!("===demo 2====");
    demo_2();
    print_line_separator();

    println!("===demo 3====");
    demo_3();
    print_line_separator();

    println!("===demo 4====");
    demo_4();
    print_line_separator();

    println!("===demo 5====");
    demo_5();
    print_line_separator();
}
