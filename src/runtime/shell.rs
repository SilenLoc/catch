use super::RuntimeError;
use crate::file_store::STORAGE_DIR;
use std::fs;
use xshell::{Shell, cmd};

pub fn run(script: &str) -> Result<String, RuntimeError> {
    let sh = Shell::new().map_err(|e| RuntimeError::InternalError(e.to_string()))?;

    // Ensure storage directory exists and change to it before executing script
    fs::create_dir_all(STORAGE_DIR).map_err(|e| {
        RuntimeError::InternalError(format!("Failed to create storage directory: {e}"))
    })?;
    sh.change_dir(STORAGE_DIR);

    // Execute the shell script using sh -c
    let output = cmd!(sh, "sh -c {script}")
        .output()
        .map_err(|e| RuntimeError::UserError(e.to_string()))?;

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)
            .map_err(|e| RuntimeError::InternalError(e.to_string()))?;
        Ok(stdout.trim_end().to_string())
    } else {
        let stderr = String::from_utf8(output.stderr)
            .map_err(|e| RuntimeError::InternalError(e.to_string()))?;
        Err(RuntimeError::UserError(stderr))
    }
}
