use std::fs::{File, exists, write};
use std::io::prelude::*;

use crate::error::BarChangerError;
use crate::error::errors::Context;

pub type Result<T> = std::result::Result<T, BarChangerError>;

pub fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path).map_err(|e| BarChangerError::Io(e))?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| BarChangerError::Io(e))?;
    Ok(content.trim().to_string())
}

pub fn write_file(path: &str, content: String) -> Result<()> {
    write(path, content).map_err(|e| BarChangerError::Io(e))?;

    Ok(())
}

pub fn file_exists(path: &str) -> Result<bool> {
    exists(path)
        .map_err(|e| BarChangerError::Io(e))
        .context("Can't check existence of file")
}

pub fn get_home_dir() -> Result<String> {
    #[cfg(unix)]
    {
        use std::process::Command;

        use crate::error::errors::Context;
        let output = Command::new("whoami")
            .output()
            .map_err(|e| BarChangerError::Io(e))
            .context("")?;
        if output.status.success() {
            let username = String::from_utf8_lossy(&output.stdout).trim().to_string();

            return Ok(format!("/home/{}", username));
        }
    }

    Err(BarChangerError::Custom("Home dir not found".to_string()))
}
