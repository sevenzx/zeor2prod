//! main.rs

use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // 0 表示让 操作系统自动分配一个空闲的端口
    // 这是 一种请求“任意可用端口”的写法
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    println!("Listening on {}", port);

    run(listener)?.await
}
