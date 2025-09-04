use std::fs::{File, exists, write};
use std::io::{self, prelude::*};

pub fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content.trim().to_string())
}

pub fn write_file(path: &str, content: String) -> Result<(), io::Error> {
    write(path, content)?;

    Ok(())
}

pub fn file_exists(path: &str) -> bool {
    exists(path).expect("Can't check existence of file")
}

pub fn get_home_dir() -> Option<String> {
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
