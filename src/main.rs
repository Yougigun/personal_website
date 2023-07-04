use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};

#[allow(unused_imports)]
use log::{debug, error, info};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(
        r#"<!DOCTYPE html>
        <html>
        <head>
            <style>
                body {
                    background-color: black;
                }
        
                .center {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                    font-size: 3em;
                    color: gold; /* Change this to any shiny color you prefer */
                }
            </style>
        </head>
        <body>
            <div class="center">
                <p>Gary's website!</p>
            </div>
        </body>
        </html>
        "#,
    )
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a %r %s %b(bytes) %D(ms)"))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run();
    info!("Starting server...");
    server.await
}
