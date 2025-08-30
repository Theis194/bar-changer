use std::fs::{File, write};
use std::io::{self, prelude::*};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    bar: String,

    #[arg(short, long)]
    style: String,
}

fn main() {
    let home_dir: String = get_home_dir().expect("Failed to get home directory");
    let waybar = ".config/waybar";

    let args = Args::parse();

    let style_name = args.style;
    let bar_name = args.bar;

    let active_style_path = format!("{}/{}/style.css", home_dir, waybar);
    let active_config_path = format!("{}/{}/config", home_dir, waybar);

    let import = format!("@import 'themes/{}.css';", style_name);

    // Read style sheet of specified waybar config
    let style = read_file(format!("{}/{}/bars/{}/style.css", home_dir, waybar, bar_name).as_str())
        .expect("Failed reading style:");

    let formatted_style = format!("{}\n{}", import, style);

    write_file(&active_style_path, formatted_style)
        .expect("Failed to write style to active style sheet");

    // Read specified config
    let config = read_file(format!("{}/{}/bars/{}/config", home_dir, waybar, bar_name).as_str())
        .expect("Failed reading file:");

    write_file(&active_config_path, config).expect("Failed to write config to active config file");

    restart_waybar();
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content.trim().to_string())
}

fn write_file(path: &str, content: String) -> Result<(), io::Error> {
    write(path, content)?;

    Ok(())
}

fn get_home_dir() -> Option<String> {
    #[cfg(unix)]
    {
        use std::process::Command;
        let output = Command::new("whoami").output().ok()?;
        if output.status.success() {
            let username = String::from_utf8_lossy(&output.stdout).trim().to_string();

            return Some(format!("/home/{}", username));
        }
    }

    None
}

fn restart_waybar() {
    if is_waybar_running() {
        println!("Waybar is running, killing it...");
        let _ = Command::new("killall").arg("waybar").output();
        thread::sleep(Duration::from_millis(500));
    }

    println!("Starting waybar...");
    let _ = Command::new("waybar")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start waybar");
}

fn is_waybar_running() -> bool {
    let output = Command::new("pgrep").arg("waybar").output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}
