use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use bar_changer::files::config::Cache;
use bar_changer::files::{read_cache, read_file, write_file};

use bar_changer::Config;
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    bar: Option<String>,

    #[arg(short, long)]
    style: Option<String>,

    #[arg(short, long, action=ArgAction::SetFalse)]
    init: bool,
}

fn main() {
    let config = Config::use_default_dirs();
    let mut cache = read_cache().unwrap();

    let args = Args::parse();

    match (&args.bar, &args.style) {
        (Some(bar), Some(style)) => {
            println!("Switching to bar: {}, style: {}", bar, style);
            set_bar(bar, &config).expect("Failed to set bar");
            set_style(bar, Some(style.clone()), &config, &cache).expect("Failed to set style");

            cache.last_bar = Some(bar.clone());
            cache.last_style = Some(style.clone());

            cache.write().expect("Failed to write cache to file");
        }
        (Some(bar), None) => {
            println!("Switching to bar: {}", bar);
            set_bar(bar, &config).expect("Failed to set bar");
            set_style(bar, None, &config, &cache).expect("Failed to set style");

            cache.last_bar = Some(bar.clone());

            cache.write().expect("Failed to write cache to file");
        }
        (None, Some(style)) => {
            println!("Switching to style: {}", style);
            change_style(style, &config).expect("Failed changing style");

            cache.last_style = Some(style.clone());

            cache.write().expect("Failed to write cache to file");
        }
        (None, None) => {
            println!("No arguments provided")
        }
    }
    restart_waybar();
}

fn change_style(style_name: &str, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let active_style_path = format!("{}/{}/style.css", config.home_dir, config.waybar_dir);

    let import = format!("@import 'themes/{}.css';", style_name);

    // Read style sheet of specified waybar config
    let raw_style = read_file(&active_style_path).expect("Failed to read active style sheet");
    let style = raw_style
        .split_once("\n")
        .map(|(_, after)| after)
        .expect("Failed to get styling");

    let formatted_style = format!("{}\n{}", import, style);

    write_file(&active_style_path, formatted_style)
        .expect("Failed to write style to active style sheet");

    Ok(())
}

fn set_bar(bar: &String, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // Read specified config
    let raw_config = read_file(
        format!(
            "{}/{}/bars/{}/config",
            config.home_dir, config.waybar_dir, bar
        )
        .as_str(),
    )
    .expect("Failed reading file:");

    let active_config_path = format!("{}/{}/config", config.home_dir, config.waybar_dir);

    write_file(&active_config_path, raw_config)
        .expect("Failed to write config to active config file");

    Ok(())
}

fn set_style(
    bar: &String,
    style: Option<String>,
    config: &Config,
    cache: &Cache,
) -> Result<(), Box<dyn std::error::Error>> {
    let style_name = style.unwrap_or_else(|| {
        cache
            .last_style
            .clone()
            .ok_or("No style has been used previously")
            .unwrap()
    });

    // Read style sheet of specified waybar config
    let style = read_file(
        format!(
            "{}/{}/bars/{}/style.css",
            config.home_dir, config.waybar_dir, bar
        )
        .as_str(),
    )
    .expect("Failed reading style:");

    let import = format!("@import 'themes/{}.css';", style_name);

    let formatted_style = format!("{}\n{}", import, style);

    let active_style_path = format!("{}/{}/style.css", config.home_dir, config.waybar_dir);

    write_file(&active_style_path, formatted_style)
        .expect("Failed to write style to active style sheet");

    Ok(())
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
