use actix_web::{web, App, HttpServer,Responder, HttpResponse};

async fn index_0()-> impl Responder{
    "Hello Index 0"
}

async fn index_1() -> impl Responder {
    HttpResponse::Ok().body("Hello Index 1")
}
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
        .route("/", web::get().to(index_0))
        .service(
            web::scope("/index1")
            .route("/hello", web::get().to(index_1))
        )
    })
    .bind("127.0.0.1:8088")?
    .run() 
    .await //Executor to wait for async function to end and return a future
}