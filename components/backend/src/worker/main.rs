mod worker_state;
mod blender;
mod executor;
mod ws;
mod messages;

use actix::prelude::*;
use actix_web::{
    web,
    web::Json,
    App,
    // HttpResponse,
    HttpServer,
    Responder,
    Result
};
use actix_files::{Files,NamedFile};
use futures::IntoFuture;
use serde::Deserialize;

// Cool tutorial:
// https://simplabs.com/blog/2018/06/27/actix-tcp-client

use worker_state::WorkerState;

use messages::{
    GetStatus,
    Status,
    StartRender
};

pub use blender::{
    Runner,
    CoolRenderUpdate
};

use ws::websocket_route;

// TODO: remove?
pub struct RunnerOptions {
    input_file: String,
    output_dir: String,
    frame_start: u32,
    frame_end: u32,
}

#[derive(Deserialize)]
struct StartJobRequest {
    frame_start: u32,
    frame_end: u32
}

fn main() {
    let sys = System::new("ruddy-worker");

    let worker_state = WorkerState::new().start();

    println!("Starting worker server on port 3200.");
    HttpServer::new(move || {
        App::new()
            .data(worker_state.clone())
            .route("/", web::get().to(index_html))
            .route("/status", web::get().to_async(status))
            .route("/jobs", web::post().to_async(start))
            .service(Files::new("/static", "./public/static").show_files_listing()) // TODO: remove the directory listing
            .route("/ws/", web::get().to(websocket_route))
    })
        .bind("127.0.0.1:3200")
        .unwrap()
        .start();

    sys.run().expect("Could not start actix System");
}

fn index_html() -> Result<NamedFile> {
    Ok(NamedFile::open("public/worker.html").unwrap())
}

fn status(state: web::Data<Addr<WorkerState>>) -> impl IntoFuture<Item = String, Error = ()> {
    state.send(GetStatus {})
        .map(|status| {
            match status {
                Status::Ready => String::from("Ready!"),
                Status::Working { job_status } => {
                    format!("Working: {:#?}", job_status)
                }
            }
        })
        .map_err(|_error| {
            // println!("[main] Error getting state!");

            ()
        })
}

fn start(job: Json<StartJobRequest>, state: web::Data<Addr<WorkerState>>) -> impl IntoFuture<Item = String, Error = ()> {
    println!("[main] POST /jobs");

    // let response_body_json: GetDropletResponse = serde_json::from_str(&response_body)
    //     .expect("Could not deserialize get droplet response");

    let render_message = StartRender {
        frame_start: job.frame_start,
        frame_end: job.frame_end
    };

    state.send(render_message)
        .map(|_status| {
            format!("Started!")
        })
        .map_err(|_error| {
            ()
        })
}
