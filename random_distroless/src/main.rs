use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use random_distroless::{coin, coin_n};

//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Coin Flipper!")
}

//create a function that flips one coin
#[get("/flip")]
async fn flip() -> impl Responder {
    //print the result from one coin flip
    println!("Result: {}", coin());
    HttpResponse::Ok().body(coin())
}

#[get("/flip10")]
async fn flip10() -> impl Responder {
    //print the result from 10 coin flips
    let result = coin_n(10);
    // convert result to string
    let result_string = result.to_string();
    println!("Result: {}", result_string);
    HttpResponse::Ok().body(result_string)
}

#[get("/flip100")]
async fn flip100() -> impl Responder {
    //print the result from 100 coin flips
    let result = coin_n(100);
    // convert result to string
    let result_string = result.to_string();
    println!("Result: {}", result_string);
    HttpResponse::Ok().body(result_string)
}

#[get("/flip1000")]
async fn flip1000() -> impl Responder {
    //print the result from 1000 coin flips
    let result = coin_n(1000);
    // convert result to string
    let result_string = result.to_string();
    println!("Result: {}", result_string);
    HttpResponse::Ok().body(result_string)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(flip)
            .service(flip10)
            .service(flip100)
            .service(flip1000)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}