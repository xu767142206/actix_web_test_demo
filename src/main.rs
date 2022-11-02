use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sqlx::mysql::MySqlPoolOptions;
use std::net::SocketAddr;
pub mod entry;
use entry::*;

#[get("/hello/{name}")]
async fn greet(
    _name: web::Path<String>,
    _query_info: web::Query<QueryRequest>,
    data: web::Data<MysqlData>,
) -> impl Responder {
    let _pool = &data.pool;

    //查询数据库
    let res = sqlx::query_as::<_, User>("SELECT * FROM wf_app_user where 1=1 limit 0,100")
        //     .bind(user_email)
        //     .bind(user_name)
        .fetch_all(_pool)
        .await
        .unwrap_or_default();

    let resdd = Result {
        data: res,
        code: 200,
        msg: String::from("fuck"),
    };
    HttpResponse::Ok().json(resdd)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(50)
        .connect("mysql://lsk_plus:zwWSCLjNHSN8WFR6@192.168.1.33/lsk_plus")
        .await
        .unwrap_or_else(|_| {
            println!("连接数据库失败!");
            std::process::exit(1)
        });

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 9527));
    println!("port:{}", addr.port());
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(MysqlData {
                app_name: String::from("mysql"),
                pool: pool.clone(),
            }))
            .service(greet)
    })
    .bind(addr)?
    .run()
    .await
}
