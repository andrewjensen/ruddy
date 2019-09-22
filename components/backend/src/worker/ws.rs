use actix::prelude::*;
use actix::{
    Addr,
    Actor,
    StreamHandler,
    fut
};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde_json::Value;

use crate::worker_state::WorkerState;
use crate::messages::{
    WorkerWsConnect,
    JobStatusUpdate
};

pub fn websocket_route(
    req: HttpRequest,
    stream: web::Payload,
    worker_state: web::Data<Addr<WorkerState>>
) -> Result<HttpResponse, Error> {
    let resp = ws::start(WsHost {
        addr: worker_state.get_ref().clone()
    }, &req, stream);

    resp
}

struct WsHost {
    addr: Addr<WorkerState>,
}

impl Actor for WsHost {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.addr
            .send(WorkerWsConnect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, _act, ctx| {
                match res {
                    Ok(_res) => {
                        println!("[Route] Connected to WorkerState");
                        // act.id = res
                    },
                    // something is wrong with WorkerState
                    _ => ctx.stop(),
                }
                fut::ok(())
            })
            .wait(ctx);
    }
}

impl Handler<JobStatusUpdate> for WsHost {
    type Result = ();

    fn handle(&mut self, msg: JobStatusUpdate, ctx: &mut Self::Context) {
        let status = msg.status;
        println!("[Route] Got status update");
        ctx.text(serde_json::to_string(&status).unwrap());
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for WsHost {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => {
                println!("Received text! {}", text);

                // Send data back to the client:
                // ctx.text(text)

                ()
            },
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}
