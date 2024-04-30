use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crate::utils::check_is_process_running; 

mod utils; 

// Endpoint to check to see if OpenSSH is running 
async fn check_ssh() -> impl Responder {
    if check_is_process_running("sshd") {
        HttpResponse::Ok().body("OpenSSH is running")
    } else {
        HttpResponse::Ok().body("OpenSSH is not running")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/check-ssh", web::get().to(check_ssh))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
