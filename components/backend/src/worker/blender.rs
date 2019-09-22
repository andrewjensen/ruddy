use std::collections::VecDeque;
use std::io::{BufRead,BufReader};
use std::process::{Command,Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use actix::prelude::*;
use futures::{
    Async,
    Poll
};
use futures::stream::Stream;
use regex::Regex;

use crate::{
    RunnerOptions
};
use crate::messages::{
    StatusUpdate
};

const BLENDER_CMD_PATH: &str = "/Applications/Blender/blender.app/Contents/MacOS/blender";

#[derive(Debug, Copy, Clone)]
pub enum CoolRenderUpdate {
    Start,
    RenderedFrame,
    End,
    NoUpdate
}

impl Message for CoolRenderUpdate {
    type Result = ();
}

#[derive(PartialEq, Debug)]
enum ParseResult {
    CurrentFrame(u32),
    SavedFrame(u32),
    FrameRenderTime(u32),
    None
}

pub struct Runner {
    exec_thread: Option<JoinHandle<()>>,
    frame_start: u32,
    frame_end: u32
}

impl Runner {
    pub fn new(frame_start: u32, frame_end: u32) -> Self {
        Self {
            exec_thread: None,
            frame_start,
            frame_end
        }
    }

    pub fn execute(&mut self) -> Execution {
        println!("[Runner] execute()\n");

        let fake_options = RunnerOptions {
            input_file: String::from("temp/example.blend"),
            output_dir: String::from("temp/frames/"),
            frame_start: self.frame_start,
            frame_end: self.frame_end
        };

        let updates: Arc<Mutex<VecDeque<StatusUpdate>>> = Arc::new(Mutex::new(VecDeque::new()));

        let updates_execution = updates.clone();
        let updates_exec = updates.clone();

        let execution = Execution::new(updates_execution);

        self.exec_thread = Some(thread::spawn(move || {
            println!("[Runner] starting blender process");
            exec_blender(Arc::new(fake_options), updates_exec);
        }));

        execution
    }
}

pub struct Execution {
    updates: Arc<Mutex<VecDeque<StatusUpdate>>>,
    finished: bool
}

impl Execution {
    pub fn new(updates: Arc<Mutex<VecDeque<StatusUpdate>>>) -> Self {
        Self {
            updates: updates,
            finished: false
        }
    }

    pub fn mark_finished(&mut self) {
        self.finished = true;
    }
}

impl Stream for Execution {
    type Item = StatusUpdate;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if self.finished {
            return Ok(Async::Ready(None))
        }

        let mut lock = self.updates.try_lock();
        if let Ok(ref mut updates_queue) = lock {
            if let Some(update) = updates_queue.pop_front() {
                if let StatusUpdate::Finished = update {
                    self.finished = true;
                }

                Ok(Async::Ready(Some(update)))
            } else {
                // TODO: Remove this hack once I figure out how to get poll() to work properly
                // Ok(Async::NotReady)
                Ok(Async::Ready(Some(StatusUpdate::NoUpdate)))
            }
        } else {
            // Mutex is not free, try again later
            // TODO: Remove this hack once I figure out how to get poll() to work properly
            // Ok(Async::NotReady)
            Ok(Async::Ready(Some(StatusUpdate::NoUpdate)))
        }
    }
}

fn exec_blender(options: Arc<RunnerOptions>, updates: Arc<Mutex<VecDeque<StatusUpdate>>>) {
    let mut blender_process = Command::new(BLENDER_CMD_PATH)
        .args(get_arguments(options))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start the child process");

    {
        updates.lock().unwrap().push_back(StatusUpdate::Started);
    }

    let mut buffered_stdout = BufReader::new(blender_process.stdout.take().unwrap());

    let mut buffer = String::new();

    let mut prev_frame: Option<u32> = None;

    while buffered_stdout.read_line(&mut buffer).unwrap() > 0 {
        let line = buffer.clone();
        buffer.clear();

        match parse_line(&line) {
            ParseResult::SavedFrame(frame_number) => {
                prev_frame = Some(frame_number);
            },
            ParseResult::FrameRenderTime(render_time) => {
                let frame_number = prev_frame.take().unwrap();
                let update = StatusUpdate::RenderedFrame {
                    frame_number,
                    render_time
                };
                updates.lock().unwrap().push_back(update);
            },
            _ => {}
        }
    }

    match blender_process.wait() {
        Ok(status) => {
            match status.code() {
                Some(exit_code) => {
                    if exit_code == 0 {
                        updates.lock().unwrap().push_back(StatusUpdate::Finished);
                    } else {
                        panic!("Blender process exited with non-zero status code: {}", exit_code);
                    }
                },
                None => {
                    panic!("Blender process terminated by signal");
                }
            }
        },
        Err(error) => {
            panic!("Blender process failed, error: {}", error);
        }
    }
}

fn get_arguments(options: Arc<RunnerOptions>) -> Vec<String> {
    vec![
        "--background".to_owned(),
        options.input_file.clone(),
        "--render-output".to_owned(),
        options.output_dir.clone(),
        "--frame-start".to_owned(),
        format!("{}", options.frame_start),
        "--frame-end".to_owned(),
        format!("{}", options.frame_end),
        "--render-anim".to_owned(),
        "--render-format".to_owned(),
        "PNG".to_owned(),
        "--use-extension".to_owned()
    ]
}

fn parse_line(line: &str) -> ParseResult {
    // TODO: use lazy_static to improve performance
    let regex_current_frame = Regex::new(r"^Fra:([0-9]+) Mem").unwrap();
    let regex_saved_frame = Regex::new(r"^Saved:.*?([0-9]+).png").unwrap();
    let regex_render_time_frame = Regex::new(r"^\s?Time: ([0-9]{2}):([0-9]{2})\.([0-9]{2})").unwrap();

    if let Some(captures) = regex_current_frame.captures(line) {
        let frame_str = &captures[1];
        let frame = frame_str.parse::<u32>().unwrap();
        return ParseResult::CurrentFrame(frame);
    }

    if let Some(captures) = regex_saved_frame.captures(line) {
        let frame_str = &captures[1];
        let frame = frame_str.parse::<u32>().unwrap();
        return ParseResult::SavedFrame(frame);
    }

    if let Some(captures) = regex_render_time_frame.captures(line) {
        let minutes_str = &captures[1];
        let minutes = minutes_str.parse::<u32>().unwrap();
        let seconds_str = &captures[2];
        let seconds = seconds_str.parse::<u32>().unwrap();
        let centiseconds_str = &captures[3];
        let centiseconds = centiseconds_str.parse::<u32>().unwrap();

        let ms_summed = (minutes * 60_000) + (seconds * 1000) + (centiseconds * 10);
        return ParseResult::FrameRenderTime(ms_summed);
    }

    ParseResult::None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_blank_line() {
        let line = "\n";
        assert_eq!(parse_line(line), ParseResult::None);
    }

    #[test]
    fn test_current_line() {
        let line = "Fra:0 Mem:16.36M (0.00M, Peak 16.37M) | Time:00:00.02 | Mem:0.00M, Peak:0.00M | Scene, RenderLayer | Synchronizing object | Cube\n";
        assert_eq!(parse_line(line), ParseResult::CurrentFrame(0));
    }

    #[test]
    fn test_current_line_alt() {
        let line = "Fra:264 Mem:18.47M (0.00M, Peak 34.43M) | Time:00:00.64 | Remaining:00:01.12 | Mem:1.87M, Peak:2.01M | Scene, RenderLayer | Path Tracing Tile 41/135\n";
        assert_eq!(parse_line(line), ParseResult::CurrentFrame(264));
    }

    #[test]
    fn test_saved_frame_line() {
        let line = "Saved: '/path/to/project/frames/0000.png'\n";
        assert_eq!(parse_line(line), ParseResult::SavedFrame(0));
    }

    #[test]
    fn test_saved_frame_line_alt() {
        let line = "Saved: '/path/to/project/frames/0058.png'\n";
        assert_eq!(parse_line(line), ParseResult::SavedFrame(58));
    }

    #[test]
    fn test_render_time() {
        // 2.19 sec == 2_190 ms
        let line = " Time: 00:02.19 (Saving: 00:00.09)\n";
        assert_eq!(parse_line(line), ParseResult::FrameRenderTime(2_190));
    }

    #[test]
    fn test_render_time_longer() {
        // 8 min, 53.97 sec == 480_000 ms + 53_970 ms == 533_970 ms
        let line = " Time: 08:53.97 (Saving: 00:00.09)\n";
        assert_eq!(parse_line(line), ParseResult::FrameRenderTime(533_970));
    }
}
