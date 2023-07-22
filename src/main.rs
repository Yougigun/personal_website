use actix_web::{
    get, http, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder,
};

#[allow(unused_imports)]
use log::{debug, error, info};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .insert_header((http::header::CACHE_CONTROL, "public, max-age=300"))
        .body(
            r#"<!DOCTYPE html>
        <html>
        <head>
            <style>
                body {
                    background-color: black;
                }
        
                .center {
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                    font-size: 3em;
                    color: gold; /* Change this to any shiny color you prefer */
                }
        
                .center button {
                    font-size: 1em;
                    margin-top: 20px;
                }
            </style>
        </head>
        <body>
            <div class="center">
                <p>Welcome to Gary's website!!</p>
                <a href="http://www.bnb.tw" target="_blank">
                    <button>Visit BNB</button>
                </a>
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

#[get("/demo/bnb")]
async fn bnb(_req_body: String) -> impl Responder {
    HttpResponse::Ok()
    .insert_header((http::header::CACHE_CONTROL, "public, max-age=300"))
    .body(r#"
    <!DOCTYPE html>
    <html>
    <head>
    <meta name="viewport" content="width=device-width, initial-scale=1">
        <style>
            body {
                background-color: #F7F7F7;
                font-family: Arial, sans-serif;
            }
    
            .container {
                width: 96%;
                margin: auto;
                display: flex;
                flex-direction: column;
                align-items: center;
                text-align: center;
            }
    
            .booking {
                display: flex;
                flex-direction: column;
                align-items: center;
                margin-bottom: 20px;
            }
    
            .room {
                width: 40%;
                margin: 20px 0;
                padding: 20px;
                box-sizing: border-box;
                border-radius: 15px;
                background-color: white;
                box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2);
                text-align: center;
            }
    
            .room img {
                max-width: 99%;
                height: auto;
                border-radius: 10px;
            }
    
            .room button {
                background-color: #FF5A5F;
                color: white;
                border: none;
                padding: 10px 20px;
                text-align: center;
                text-decoration: none;
                display: inline-block;
                font-size: 16px;
                margin: 10px 2px;
                cursor: pointer;
                border-radius: 4px;
            }
    
            .booking div {
                margin-bottom: 10px;
            }
    
            /* Media query for screens smaller than 600px */
            @media only screen and (max-width: 600px) {
                .container {
                    width: 95%;
                }
                .room {
                    width: 95%;
                }
                .room img {
                    max-width: 90%;
                }
            }
        </style>
    </head>
    <body>
        <div class="container">
            <h1>Welcome to my demo bnb!</h1>
            <div class="booking">
                <div>
                    <label for="date">Select date:</label>
                    <input type="date" id="date" name="date">
                </div>
                <div>
                    <p id="available">Available rooms: 10</p>
                </div>
            </div>
            <div class="room">
                <img src="https://static.owlting.com/booking/image/h/4396e57d-d87f-4f13-b353-ac6fffbe0ff0/images/5HRPDjpiBb4RN8fcRzOgGKvwhnkhYTOsxmknKzA1.jpeg" alt="Room 1">
                <h2>Room 1</h2>
                <p>Details about the room...</p>
                <a href="http://www.bnb.tw" target="_blank"><button>Book Now</button></a>
            </div>
            <!-- Add more rooms as needed -->
        </div>
    
        <script>
            // This is just a simple script to randomly update the number of available rooms
            const available = document.getElementById('available');
    
            document.getElementById('date').addEventListener('change', function() {
                const numAvailable = Math.floor(Math.random() * 11); // random number between 0 and 10
                available.textContent = `Available rooms: ${numAvailable}`;
            });
        </script>
    </body>
    </html>
    
    "#
    ,)
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
            .service(bnb)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run();
    info!("Starting server...");
    server.await
}
