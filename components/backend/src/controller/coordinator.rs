use actix::prelude::*;
use actix::{
    Recipient
};

use crate::WorkerClient;
use crate::messages::{
    Connect,
    ControllerState,
    WorkerCommand,
    StatusUpdate,
    CreateWorker
};
use crate::uuid;


// This is following the pattern here:
// https://github.com/actix/examples/blob/master/websocket-chat/src/server.rs

pub struct Coordinator {
    listeners: Vec<Recipient<StatusUpdate>>,
    workers: Vec<Recipient<WorkerCommand>>,
    state: ControllerState
}

impl Default for Coordinator {
    fn default() -> Coordinator {
        Coordinator {
            listeners: Vec::new(),
            workers: Vec::new(),
            state: ControllerState {
                jobs: Vec::new()
            },
        }
    }
}

impl Actor for Coordinator {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("[Coordinator] Started");

        // ctx.run_later(Duration::from_millis(1000), move |act, _| {
        //     act.status.jobs.push(JobStatus {
        //         frame_start: 0,
        //         frame_end: 1000,
        //         render_times: Vec::new(),
        //         // time_start: None,
        //         // time_end: None
        //     });
        // });

        // TODO: get rid of this debugging
        // ctx.notify(CreateWorker {});
    }
}

impl Handler<Connect> for Coordinator {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Context<Self>) {
        println!("[Coordinator] Someone joined");

        self.listeners.push(msg.addr);

        self.send_state_update();
    }
}

impl Handler<CreateWorker> for Coordinator {
    type Result = ();

    fn handle(&mut self, _msg: CreateWorker, ctx: &mut Context<Self>) {
        println!("[Coordinator] Creating worker...");

        self.create_worker(ctx);
    }
}

impl Coordinator {
    fn create_worker(&mut self, ctx: &mut Context<Self>) {
        let job_id = uuid::generate_uuid();

        let coordinator_address = ctx.address();
        let worker_arbiter = Arbiter::new();
        let worker_client = WorkerClient::start_in_arbiter(&worker_arbiter, |_ctx: &mut Context<WorkerClient>| {
            WorkerClient::new(job_id, coordinator_address)
        });

        self.workers.push(worker_client.clone().recipient());

        worker_client.do_send(WorkerCommand::Create);
    }

    fn send_state_update(&mut self) {
        println!("[Coordinator] Sending state update to everyone");

        for listener in self.listeners.iter() {
            println!("[Coordinator]  Sending to recipient");
            listener
                .do_send(StatusUpdate {
                    status: self.state.clone()
                })
                .unwrap();
        }
    }
}
