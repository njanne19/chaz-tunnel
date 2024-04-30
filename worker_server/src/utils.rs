use std::process::Command; 

pub fn check_is_process_running(process_name: &str) -> bool {
    let output = Command::new("pgrep").arg(process_name).output().unwrap(); 
    output.status.success()
}