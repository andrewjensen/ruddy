mod messages;
mod ws;
mod coordinator;
mod worker_client;
mod uuid;

use actix::*;
use actix_web::{
    web,
    App,
    // HttpResponse,
    HttpServer,
    // Responder,
    Result
};
use actix_files::{Files,NamedFile};

pub use coordinator::Coordinator;
pub use worker_client::{
    WorkerClient
};
use ws::websocket_route;

fn main() {
    let sys = System::new("ruddy-controller");

    let coordinator = Coordinator::default().start();

    println!("Starting controller server on port 3100.");
    HttpServer::new(move || {
        App::new()
            .data(coordinator.clone())
            .route("/", web::get().to(index_html))
            .service(Files::new("/static", "./public/static").show_files_listing()) // TODO: remove the directory listing
            .route("/ws/", web::get().to(websocket_route))
    })
        .bind("127.0.0.1:3100")
        .unwrap()
        .start();

    sys.run().expect("Could not start actix System");
}

fn index_html() -> Result<NamedFile> {
    Ok(NamedFile::open("public/controller.html").unwrap())
}
