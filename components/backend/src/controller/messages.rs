use actix::prelude::*;
use serde::Serialize;

// use chrono::prelude::*;
// use std::time::Duration;

#[derive(Message)]
pub struct StatusUpdate {
    pub status: ControllerState
}

// #[derive(Message)]
pub enum WorkerCommand {
    Create,
    Render,
    Halt,
    Destroy
}

impl Message for WorkerCommand {
    // type Result = String;
    type Result = Result<String, String>;
}

#[derive(Message)]
pub struct Connect {
    pub addr: Recipient<StatusUpdate>,
}

#[derive(Message)]
pub struct CreateWorker {

}

#[derive(Debug, Clone, Serialize)]
pub struct ControllerState {
    pub jobs: Vec<Job>
}

#[derive(Debug, Clone, Serialize)]
pub struct Job {

}

#[derive(Debug, Clone, Serialize)]
struct JobStatus {
    frame_start: u32,
    frame_end: u32,
    render_times: Vec<u32>,
    // time_start: Option<DateTime<Local>>,
    // time_end: Option<DateTime<Local>>
}
