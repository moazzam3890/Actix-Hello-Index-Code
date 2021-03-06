use actix_web::{web, App, HttpServer,Responder, HttpResponse};

async fn index_0()-> impl Responder{ //Request handler
    "Hello Index 0"
}

async fn index_1() -> impl Responder { //Request handler
    HttpResponse::Ok().body("Hello Index 1") 
}
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new() //create new application builder
        .route("/", web::get().to(index_0))//configure index_0 (route) for specific path ("/")
        .service( //Register http service 
            web::scope("/index1") //set of resources with common route path
            .route("/hello", web::get().to(index_1)) //
        )
    })
    .bind("127.0.0.1:8088")? // bind to the local host
    .run() // start new thread and start listning for incoming connections
    .await //Executor to wait for async function to end and return a future
}