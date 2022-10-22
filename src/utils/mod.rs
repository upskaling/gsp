use psutil::process::processes;

pub mod textutils;

pub fn get_pidof(process_name: &str) -> Vec<u32> {
    let mut processes_list = Vec::new();
    for process in processes().unwrap() {
        let process = process.unwrap();

        if process.name().unwrap() == process_name {
            processes_list.push(process.pid());
        }
    }

    processes_list
}
