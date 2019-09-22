use std::error;
use std::fmt;
use actix::prelude::*;
// use actix::fut::wrap_future;
// use actix_web::client::Client;
// use futures::stream;

use crate::coordinator::Coordinator;
use crate::messages::{
    WorkerCommand
};

// #[derive(Message)]
// pub struct ProvisionServer;

pub struct WorkerClient {
    pub job_id: String,
    pub coordinator: Addr<Coordinator>
}

impl WorkerClient {
    pub fn new(job_id: String, coordinator: Addr<Coordinator>) -> Self {
        Self {
            job_id,
            coordinator
        }
    }
}

impl Actor for WorkerClient {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("[WorkerClient] Started");
    }
}

// https://actix.rs/book/actix/sec-2-actor.html
// https://doc.rust-lang.org/beta/rust-by-example/error/multiple_error_types/define_error_type.html
// https://docs.rs/actix/0.8.3/actix/fut/trait.ActorFuture.html
// https://github.com/actix/actix/tree/master/examples/chat/src

#[derive(Debug)]
struct CreateWorkerError {

}

impl fmt::Display for CreateWorkerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error happened")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for CreateWorkerError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl Handler<WorkerCommand> for WorkerClient {
    // type Result = ();
    type Result = Result<String, String>;

    fn handle(&mut self, msg: WorkerCommand, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            WorkerCommand::Create => {
                println!("[WorkerClient] Received command to create");
                println!("  TODO: call DigitalOcean, provision server, poll endpoint");

                Ok(String::from("Hi"))
            },
            _ => panic!("[WorkerClient] Command not implemented!")
        }
    }
}


// impl Handler<WorkerCommand> for WorkerClient {
//     // type Result = ResponseActFuture<Self, (), CreateWorkerError>;
//     // type Result = ();
//     // type Result = String;
//     type Result = ResponseActFuture<Self, String, String>;

//     fn handle(&mut self, msg: WorkerCommand, _ctx: &mut Context<Self>) -> Self::Result {
//         match msg {
//             WorkerCommand::Create => {
//                 println!("[WorkerClient] Received command to create");

//                 println!("[WorkerClient] Sending request...");

//                 let execution = Client::new()
//                     .get("https://en.wikipedia.org/wiki/Synthesizer")   // <- Create request builder
//                     .header("User-Agent", "Actix-web")
//                     .send()
//                     .map_err(|_| format!("Something broke"))
//                     .and_then(|response| {
//                         format!("Got response: {:?}", response)
//                     });

//                     // .map_err(|_| format!("Something broke"))
//                     // .and_then(|mut response| response.body())
//                     // .map_err(|_| format!("Something broke"))
//                     // .and_then(move |bytes| {
//                     //     let s = std::str::from_utf8(&bytes).expect("utf8 parse error)");

//                     //     futures::done::<String, String>(Ok(format!("html: {:?}", s)))
//                     // })
//                     // .map_err(|_| format!("Something broke"));


//                 Box::new(wrap_future::<_, Self>(execution))
//             },
//             _ => panic!("[WorkerClient] Command not implemented!")
//         }
//     }

// }
