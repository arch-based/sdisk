use std::env;
use std::process::{Command, exit};
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Check for flags in arguments
    for arg in &args {
        if arg.starts_with("-") || arg.starts_with("--") || arg.starts_with("/dev/") {
            eprintln!("Error: Flags and other devices are not supported.");
            exit(1);
        }
    }

    let disk = "/dev/sda"; // Default disk for SATA drives

    let command = format!("lsblk -f {} | grep 'G'", disk);

    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .unwrap_or_else(|e| {
            eprintln!("Failed to execute command: {}", e);
            exit(1);
        });

    if !output.status.success() {
        eprintln!("Command executed with error: {}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }

    let output = str::from_utf8(&output.stdout).unwrap();
    let lines: Vec<&str> = output.lines().collect();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            let disk_space = parts[3];
            let disk_percentage = parts[4].trim_end_matches('%');
            println!("Disk Space: {}iB", disk_space);
            println!("Disk Percentage: {}%", disk_percentage);
        }
    }
}
