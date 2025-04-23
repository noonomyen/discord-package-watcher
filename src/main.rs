use std::env;
use std::path::Path;
use std::process::{Command, Stdio, exit};
use std::sync::mpsc;

use notify::{Event, EventKind, RecursiveMode, Result, Watcher, event, recommended_watcher};

fn prompt_install(path: &str) -> bool {
    let result = Command::new("zenity")
        .args(&[
            "--question",
            "--title",
            "Discord Package Watcher",
            "--text",
            &format!(
                "A new .deb file was detected in\n\n{}\n\nWould you like to install it?",
                path
            ),
        ])
        .output();

    match result {
        Ok(output) => {
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                println!("[STDOUT] zenity: {}", line);
            }
            for line in String::from_utf8_lossy(&output.stderr).lines() {
                println!("[STDERR] zenity: {}", line);
            }
            println!("[INFO] zenity: status {}", &output.status);
            output.status.success()
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("[ERROR] 'zenity' not found. Please install it to use GUI prompts.");
            false
        }
        Err(e) => {
            eprintln!("[ERROR] zenity: {}", e);
            false
        }
    }
}

fn deb_install(path: &str) -> bool {
    let result = Command::new("pkexec")
        .args(&["dpkg", "--install", path])
        .output();

    match result {
        Ok(output) => {
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                println!("[STDOUT] pkexec dpkg: {}", line);
            }
            for line in String::from_utf8_lossy(&output.stderr).lines() {
                println!("[STDERR] pkexec dpkg: {}", line);
            }
            println!("[INFO] pkexec dpkg: status {}", &output.status);
            output.status.success()
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("[ERROR] 'pkexec' not found. Please install it to run with privileges.");
            false
        }
        Err(e) => {
            eprintln!("[ERROR] pkexec dpkg: {}", e);
            false
        }
    }
}

fn start_discord() -> std::io::Result<()> {
    Command::new("setsid")
        .args(&["gtk-launch", "discord"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <watching path>", args[0]);
        exit(0);
    }

    let watching_path = Path::new(&args[1]);

    if !watching_path.exists() {
        eprintln!("Path '{}' does not exist.", watching_path.display());
        exit(1);
    } else if !watching_path.is_dir() {
        eprintln!("Path '{}' is not a directory.", watching_path.display());
        exit(1);
    }

    ctrlc::set_handler(|| {
        println!("\n[INFO] Stopping watcher.");
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = recommended_watcher(tx)?;

    watcher.watch(watching_path, RecursiveMode::NonRecursive)?;

    println!(
        "[INFO] Discord package watcher is watching at: {}",
        watching_path.display()
    );

    for res in rx {
        match res {
            Ok(event) if matches!(event.kind, EventKind::Create(event::CreateKind::File)) => {
                for path in event.paths.iter() {
                    if let Some(ext) = path.extension() {
                        if ext == "deb" {
                            if let Some(file_name) = path.file_name().and_then(|name| name.to_str())
                            {
                                if file_name.starts_with("discord") {
                                    println!("[INFO] Detected new .deb file: '{}'", path.display());

                                    let path_str = path.to_string_lossy();
                                    if prompt_install(&path_str) && deb_install(&path_str) {
                                        println!(
                                            "[INFO] Installation succeeded for '{}'",
                                            path.display()
                                        );
                                        println!("[INFO] Starting discord");

                                        if let Err(e) = start_discord() {
                                            eprintln!("[ERROR] Failed to start discord: {}", e);
                                        }
                                    } else {
                                        println!(
                                            "[INFO] Installation canceled or failed for '{}'",
                                            path.display()
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}
