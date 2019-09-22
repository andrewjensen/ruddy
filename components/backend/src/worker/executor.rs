use actix::prelude::*;

use crate::WorkerState;
use crate::messages::StatusUpdate;

pub struct Executor {
    worker_state: Addr<WorkerState>
}

impl Executor {
    pub fn new(worker_state: Addr<WorkerState>) -> Executor {
        Executor {
            worker_state
        }
    }
}

impl Actor for Executor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("[Executor] Started");
    }
}

impl StreamHandler<StatusUpdate, ()> for Executor {
    fn handle(&mut self, msg: StatusUpdate, _: &mut Context<Executor>) {
        match msg {
            StatusUpdate::NoUpdate => {},
            _ => {
                self.worker_state.do_send(msg);
            }
        }
    }

    fn error(&mut self, _: (), _: &mut Context<Executor>) -> Running {
        println!("[Executor] got error from stream");

        Running::Continue
    }

    fn finished(&mut self, _: &mut Context<Executor>) {
        println!("[Executor] finished with stream");
    }
}
