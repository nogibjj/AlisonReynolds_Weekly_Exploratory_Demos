// Build a actix microservice

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Dinner Service")
}

// return a random breakfast food
#[get("/breakfast")]
async fn breakfast_time() -> impl Responder {
    let result = time_to_eat::random_breakfast();
    HttpResponse::Ok().body(result)
}

// return a random lunch food
#[get("/lunch")]
async fn lunch_time() -> impl Responder{
    let result = time_to_eat::random_lunch();
    HttpResponse::Ok().body(result)
}

// return a random snack food
#[get("/snack")]
async fn snack_time() -> impl Responder {
    let result = time_to_eat::random_snack();
    HttpResponse::Ok().body(result)
}

// return a random dinner food
#[get("/dinner")]
async fn dinner_time() -> impl Responder{
    let result = time_to_eat::random_dinner();
    HttpResponse::Ok().body(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(breakfast_time)
            .service(lunch_time)
            .service(snack_time)
            .service(dinner_time)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
