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

use crate::coordinator::Coordinator;
use crate::messages::{
    Connect,
    StatusUpdate,
    CreateWorker
};

pub fn websocket_route(
    req: HttpRequest,
    stream: web::Payload,
    coordinator: web::Data<Addr<Coordinator>>
) -> Result<HttpResponse, Error> {
    let resp = ws::start(WsHost {
        addr: coordinator.get_ref().clone()
    }, &req, stream);

    resp
}

struct WsHost {
    addr: Addr<Coordinator>,
}

impl Actor for WsHost {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.addr
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, _act, ctx| {
                match res {
                    Ok(_res) => {
                        println!("[Route] Connected to Coordinator");
                        // act.id = res
                    },
                    // something is wrong with Coordinator
                    _ => ctx.stop(),
                }
                fut::ok(())
            })
            .wait(ctx);
    }
}

impl Handler<StatusUpdate> for WsHost {
    type Result = ();

    fn handle(&mut self, msg: StatusUpdate, ctx: &mut Self::Context) {
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
                // println!("Received text! {}", text);

                let command: Value = serde_json::from_str(&text)
                    .expect("Unable to parse command JSON");

                if let Value::Object(command_obj) = command {
                    if let Some(Value::String(type_str)) = command_obj.get("type") {
                        if type_str == "COMMAND_CREATE_WORKER" {
                            println!("[Route] Received command to create a worker!");
                            self.addr.do_send(CreateWorker {});
                        }
                    }
                }

                // Send data back to the client:
                // ctx.text(text)

                ()
            },
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}
