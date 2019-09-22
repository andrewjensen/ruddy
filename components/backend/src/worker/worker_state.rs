use actix::prelude::*;

use crate::executor::Executor;
use crate::blender::Runner;
use crate::messages::{
    WorkerWsConnect,
    GetStatus,
    StartRender,
    JobStatusUpdate,
    StatusUpdate,
    JobStatus,
    Status
};

pub struct WorkerState {
    pub started: bool,
    job_status: Option<JobStatus>,
    listeners: Vec<Recipient<JobStatusUpdate>>
}

impl WorkerState {
    pub fn new() -> Self {
        Self {
            started: false,
            job_status: None,
            listeners: Vec::new()
        }
    }

    fn update_clients(&self) {
        println!("[WorkerState] notify_clients()");

        if let None = self.job_status {
            return;
        }

        // TODO: too much cloning
        let status_to_send = JobStatusUpdate {
            status: Status::Working {
                job_status: self.job_status.clone().unwrap()
            }
        };

        for listener in self.listeners.iter() {
            println!("[WorkerState] Sending to recipient");
            listener
                .do_send(status_to_send.clone())
                .unwrap();
        }
    }
}

impl Actor for WorkerState {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("[WorkerState] Started");
    }
}

impl Handler<WorkerWsConnect> for WorkerState {
    type Result = ();

    fn handle(&mut self, msg: WorkerWsConnect, _ctx: &mut Context<Self>) {
        println!("[WorkerState] Someone joined");

        self.listeners.push(msg.addr);

        self.update_clients();
    }
}

impl Handler<GetStatus> for WorkerState {
    type Result = Status;

    fn handle(&mut self, _msg: GetStatus, _ctx: &mut Context<Self>) -> Self::Result {
        match self.job_status {
            Some(ref job_status) => {
                Status::Working {
                    job_status: job_status.clone()
                }
            },
            None => {
                Status::Ready
            }
        }
    }
}

impl Handler<StartRender> for WorkerState {
    type Result = ();

    fn handle(&mut self, msg: StartRender, ctx: &mut Context<Self>) {
        println!("[WorkerState] Starting render, from frame {} to frame {}", msg.frame_start, msg.frame_end);

        let frame_start = msg.frame_start;
        let frame_end = msg.frame_end;
        let frames_to_render = frame_end - frame_start + 1;

        self.job_status = Some(JobStatus {
            frame_start,
            frame_end,
            frames_to_render,
            render_times: Vec::new()
        });

        let worker_state_address = ctx.address();
        let executor_arbiter = Arbiter::new();

        Executor::start_in_arbiter(&executor_arbiter, move |ctx: &mut Context<Executor>| {
            let mut runner = Runner::new(frame_start, frame_end);
            let execution = runner.execute();
            Executor::add_stream(execution, ctx);
            Executor::new(worker_state_address)
        });
    }
}

impl Handler<StatusUpdate> for WorkerState {
    type Result = ();

    fn handle(&mut self, msg: StatusUpdate, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            StatusUpdate::Started => {
                println!("[WorkerState] Render started!");
            },
            StatusUpdate::RenderedFrame { frame_number, render_time } => {
                println!("[WorkerState] Rendered frame {} in {} ms", frame_number, render_time);
                let job_status = self.job_status.as_mut().unwrap();
                job_status.render_times.push(render_time);
                self.update_clients();
            },
            StatusUpdate::Finished => {
                println!("[WorkerState] Finished!");
                println!("[WorkerState] {:#?}", self.job_status.as_ref().unwrap());
            },
            _ => unreachable!()
        }
    }
}
