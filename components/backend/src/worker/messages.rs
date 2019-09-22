use actix::prelude::*;
use serde::Serialize;

#[derive(Message)]
pub struct StartRender {
    pub frame_start: u32,
    pub frame_end: u32
}

#[derive(Message)]
pub struct WorkerWsConnect {
    pub addr: Recipient<JobStatusUpdate>,
}

// TODO: combine with next message?
// TODO: rename to be ProcessUpdate or something
#[derive(Debug)]
pub enum StatusUpdate {
    Started,
    RenderedFrame {
        frame_number: u32,
        render_time: u32,
    },
    Finished,

    // TODO: Remove this hack once I figure out how to get poll() to work properly
    NoUpdate
}

impl Message for StatusUpdate {
    type Result = ();
}

#[derive(Message, Clone)]
pub struct JobStatusUpdate {
    pub status: Status
}

pub struct GetStatus {

}

impl Message for GetStatus {
    type Result = Status;
}

#[derive(Clone, MessageResponse, Serialize)]
pub enum Status {
    Ready,
    Working {
        job_status: JobStatus
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct JobStatus {
    pub frame_start: u32,
    pub frame_end: u32,
    pub frames_to_render: u32,
    pub render_times: Vec<u32>,
    // pub time_start: Option<DateTime<Local>>,
    // pub time_end: Option<DateTime<Local>>
}
