use std::{
    fs,
    process::Command,
};

use heapless::String;

use crate::common::{
    self,
};

pub fn run_man(on_file: &str) -> Option<std::string::String> {
    match Command::new("man").arg(on_file).arg("--pager=cat").output() {
        Ok(output) if !output.stdout.is_empty() => Some(std::string::String::from_utf8_lossy(&output.stdout).to_string()),
        _ => None,
    }
}

pub fn run_command(command: &str) -> Option<std::string::String> {
    let status = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status();

    match status {
        Ok(_) => Some(std::string::String::new()),
        Err(_) => None,
    }
}

pub fn search_in_directories(dirs: &[&str], filter: Option<&str>) -> Vec<String<{ common::STRING_SIZE }>> {
    let mut buffer = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for dir in dirs {
        if let Ok(es) = fs::read_dir(dir) {
            for e in es.flatten() {
                if let Ok(file_name) = e.file_name().into_string() {
                    if let Some(ref f) = filter
                        && !file_name.contains(f)
                    {
                        continue;
                    }

                    if seen.contains(&file_name) {
                        continue;
                    }

                    let mut s: String<{ common::STRING_SIZE }> = String::new();

                    if s.push_str(&file_name).is_ok() {
                        seen.insert(file_name);
                        buffer.push(s);
                    }
                }
            }
        }
    }

    buffer
}
