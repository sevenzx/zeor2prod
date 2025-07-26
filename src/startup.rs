use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            // tracing_actix_web 提供的中间件，用于收集和记录 HTTP 请求日志 生成request_id
            // 这个中间件只是 产生日志事件，但这些日志要真正输出到终端（stdout），就必须初始化一个具体的日志实现
            .wrap(TracingLogger::default()) // 日志中间件
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    // .await

    Ok(server)
}
