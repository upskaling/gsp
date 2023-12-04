use psutil::process::{processes, Process, ProcessError};

pub mod textutils;

pub fn get_pidof(process_name: &str) -> Vec<u32> {
    let processes = processes().unwrap().into_iter();

    processes
        .filter_map(|p| get_pid(p, process_name))
        .collect::<Vec<u32>>()
}

fn get_pid(process: Result<Process, ProcessError>, process_name: &str) -> Option<u32> {
    let p = match process {
        Ok(p) => p,
        Err(_) => return None,
    };

    if p.name().unwrap_or_else(|_| "".to_string()) == process_name {
        return Some(p.pid());
    }

    None
}
