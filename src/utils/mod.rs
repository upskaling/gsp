// Vérifier si une autre instance du programme est en cours

pub fn get_pid(process_name: &str) -> bool {
    let cmd = std::process::Command::new("pidof")
        .arg(process_name)
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&cmd.stdout);
    let output = output.trim();

    let pid = std::process::id().to_string();

    return output != pid;
}
