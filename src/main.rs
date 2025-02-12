use crate::constructx::{get_base_user, Rectangle};
use crate::controller::get_base_first_word;
use crate::migrate::*;
use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv;

mod migrate;
mod controller;
mod constructx;
mod enumx;
mod packagex;
mod arraylistx;
mod Typex;
mod testx;
mod iterator;
mod boxr;
mod smartPointer;
mod threadx;
mod db;

fn main() {
    /*get_base_tuple();
    get_base_array();
    let success = get_else_if(6);
    println!("success: {}", success);
    get_loop();*/

    // let s = String::from("hello");
    // get_base_first_word(&s);

    // get_base_user();
    /* let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    ); */

    // boxr::test_box();

    // smartPointer::test_pointer();
    threadx::get_thread();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::create_pool().await.expect("数据库连接失败");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // ...其他配置
    })
    // ...启动服务
}
