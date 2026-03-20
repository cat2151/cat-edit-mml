use std::process::Command;

const GIT_URL: &str = "https://github.com/cat2151/cat-edit-mml.git";

fn install_cmd() -> String {
    format!("cargo install --force --git {GIT_URL}")
}

#[cfg(any(target_os = "windows", test))]
pub(crate) fn update_bat_content() -> String {
    format!(
        "@echo off\r\ntimeout /t 3 /nobreak >nul\r\n{cmd}\r\ndel \"%~f0\"\r\n",
        cmd = install_cmd()
    )
}

pub fn run_self_update() -> anyhow::Result<bool> {
    #[cfg(target_os = "windows")]
    {
        use std::io::Write;
        use std::time::{SystemTime, UNIX_EPOCH};

        let pid = std::process::id();
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let bat_path = std::env::temp_dir().join(format!("cat_edit_mml_update_{pid}_{ts}.bat"));
        {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&bat_path)?;
            file.write_all(update_bat_content().as_bytes())?;
        }

        let bat_str = bat_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("temp bat path is not valid UTF-8"))?;
        Command::new("cmd")
            .args(["/C", "start", "", bat_str])
            .spawn()?;

        println!("Launching update script: {}", bat_path.display());
        println!("The application will now exit so the file lock is released.");
        return Ok(true);
    }

    #[cfg(not(target_os = "windows"))]
    {
        let cmd = install_cmd();
        println!("Running: {cmd}");
        let status = Command::new("cargo")
            .args(["install", "--force", "--git", GIT_URL])
            .status()?;
        if !status.success() {
            anyhow::bail!("cargo install failed with status: {status}");
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bat_content_contains_install_command() {
        let bat = update_bat_content();
        assert!(bat.contains("cargo install --force --git"));
        assert!(bat.contains("cat-edit-mml.git"));
    }

    #[test]
    fn bat_content_has_delay() {
        let bat = update_bat_content();
        assert!(bat.contains("timeout"));
    }

    #[test]
    fn bat_content_self_deletes() {
        let bat = update_bat_content();
        assert!(bat.contains("del"));
        assert!(bat.contains("%~f0"));
    }
}
