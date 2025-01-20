use std::env;
use std::process::{Command, exit};

#[cfg(target_family = "windows")]
const NETSTAT_CMD: &str = "netstat";
#[cfg(target_family = "unix")]
const NETSTAT_CMD: &str = "lsof";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <port> <command>", args[0]);
        eprintln!("Commands:");
        eprintln!("  inspect - Show the program using the specified port");
        eprintln!("  kill    - Kill the process using the specified port");
        exit(1);
    }

    let port = &args[1];
    let command = &args[2];

    match command.as_str() {
        "inspect" => inspect_port(port),
        "kill" => kill_port(port),
        _ => {
            eprintln!("Invalid command: {}", command);
            eprintln!("Available commands: inspect, kill");
            exit(1);
        }
    }
}

fn inspect_port(port: &str) {
    #[cfg(target_family = "unix")]
    {
        let output = Command::new(NETSTAT_CMD)
            .arg("-i")
            .arg(format!(":{}", port))
            .output()
            .expect("Failed to execute lsof command");

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            if result.is_empty() {
                println!("No process is using port {}", port);
            } else {
                println!("Processes using port {}:\n{}", port, result);
            }
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error: {}", error);
        }
    }

    #[cfg(target_family = "windows")]
    {
        let output = Command::new(NETSTAT_CMD)
            .arg("-ano")
            .output()
            .expect("Failed to execute netstat command");

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = result.lines().collect();

            let matching_lines: Vec<&str> = lines
                .iter()
                .filter(|line| line.contains(&format!(":{}", port)))
                .map(|line| *line)
                .collect();

            if matching_lines.is_empty() {
                println!("No process is using port {}", port);
            } else {
                println!("Processes using port {}:\n{}", port, matching_lines.join("\n"));
            }
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error: {}", error);
        }
    }
}

fn kill_port(port: &str) {
    #[cfg(target_family = "unix")]
    {
        let output = Command::new(NETSTAT_CMD)
            .arg("-i")
            .arg(format!(":{}", port))
            .output()
            .expect("Failed to execute lsof command");

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            if result.is_empty() {
                println!("No process is using port {}", port);
                return;
            }

            let lines: Vec<&str> = result.lines().collect();
            if lines.len() > 1 {
                let pid: Vec<&str> = lines[1].split_whitespace().collect();
                if let Some(pid) = pid.get(1) {
                    println!("Killing process with PID: {}", pid);
                    let kill_output = Command::new("kill")
                        .arg("-9")
                        .arg(pid)
                        .output()
                        .expect("Failed to execute kill command");

                    if kill_output.status.success() {
                        println!("Successfully killed process {}", pid);
                    } else {
                        let error = String::from_utf8_lossy(&kill_output.stderr);
                        eprintln!("Failed to kill process {}: {}", pid, error);
                    }
                }
            }
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error: {}", error);
        }
    }

    #[cfg(target_family = "windows")]
    {
        let output = Command::new(NETSTAT_CMD)
            .arg("-ano")
            .output()
            .expect("Failed to execute netstat command");

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = result.lines().collect();

            let matching_lines: Vec<&str> = lines
                .iter()
                .filter(|line| line.contains(&format!(":{}", port)))
                .map(|line| *line)
                .collect();

            if let Some(line) = matching_lines.first() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(pid) = parts.last() {
                    println!("Killing process with PID: {}", pid);
                    let kill_output = Command::new("taskkill")
                        .arg("/F")
                        .arg("/PID")
                        .arg(pid)
                        .output()
                        .expect("Failed to execute taskkill command");

                    if kill_output.status.success() {
                        println!("Successfully killed process {}", pid);
                    } else {
                        let error = String::from_utf8_lossy(&kill_output.stderr);
                        eprintln!("Failed to kill process {}: {}", pid, error);
                    }
                }
            } else {
                println!("No process is using port {}", port);
            }
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error: {}", error);
        }
    }
}