use std::collections::hash_map::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::*;
use std::process::Command;
use super::temp_file::TempFile;

pub struct DOSBox {
    dosbox: PathBuf,
    cwd: PathBuf,
    command: String,
    batch_env: HashMap<String, String>,
}

impl DOSBox {
    pub fn new() -> Self {
        Self {
            dosbox: PathBuf::new(),
            cwd: PathBuf::new(),
            command: String::new(),
            batch_env: HashMap::new(),
        }
    }

    pub fn dosbox<T>(&mut self, dosbox: T) -> &mut Self where PathBuf: From<T> {
        self.dosbox = PathBuf::from(dosbox);
        self
    }

    pub fn find_dosbox(&mut self) -> Result<&mut Self, String> {
        match super::find::find_dosbox() {
            Some(dosbox) => Ok(self.dosbox(dosbox)),
            None => Err("Could not find dosbox in PATH".to_string())
        }
    }

    pub fn cwd<T>(&mut self, cwd: T) -> &mut Self where PathBuf: From<T> {
        self.cwd = PathBuf::from(cwd);
        self
    }

    pub fn command<T>(&mut self, command: T) -> &mut Self where String : From<T> {
        self.command = String::from(command);
        self
    }

    pub fn env<V>(&mut self, name: &str, value: V) -> &mut Self where String : From<V> {
        self.batch_env.insert(name.to_string(), String::from(value));
        self
    }

    pub fn pass_through_env(&mut self, name: &str) -> &mut Self {
        self.env(name, env::var(name).unwrap_or_default())
    }

    pub fn run(&mut self) -> Result<(), String> {
        if !self.cwd.is_dir() {
            return Err(format!("Could not find run directory {}", self.cwd.display()));
        }

        if self.command.is_empty() {
            return Err("Command not given".to_string());
        }

        let stdout_file_name = "OUT.TXT";
        let stdout_file = TempFile::from(self.cwd.join(stdout_file_name));
        let batch_file = TempFile::from(self.cwd.join("WRAP.BAT"));
        self.create_batch_wrapper(stdout_file_name, batch_file.as_ref()).unwrap();

        let dosbox_conf = TempFile::from(self.cwd.join("dosbox.conf"));
        create_minimal_dosbox_config(dosbox_conf.as_ref()).unwrap();

        let result = Command::new(&self.dosbox)
            .args(&[batch_file.to_str().unwrap(), "-exit", "-noconsole", "-noautoexec", "-conf", dosbox_conf.to_str().unwrap()])
            .env_clear()
            .env("SDL_VIDEODRIVER", "dummy")
            .env("TERM", "dumb")
            .output()
            .expect("Error running DOSBox");
        if !result.status.success() {
            return Err("DOSBox did not return a success error code".to_string());
        }
        let out = fs::read(&stdout_file).unwrap();
        io::stdout().write_all(&out).unwrap();
        Ok(())
    }

    fn create_batch_wrapper(&self, stdout_file_name: &str, batch_file: &Path) -> Result<(), std::io::Error> {
        let mut f = File::create(batch_file)?;
        write!(f, "@ECHO OFF\r\n")?;
        for (key, value) in &self.batch_env {
            write!(f, "SET {}={}\r\n", key, value)?;
        }
        // switch to C: drive
        write!(f, "C:\r\n")?;
        write!(f, "{} > {}\r\n", self.command, stdout_file_name)?;
        Ok(())
    }

    pub fn customize<F>(&mut self, mut f: F) -> &mut Self where F : FnMut(&mut Self) -> &mut Self {
        f(self)
    }
}

fn create_minimal_dosbox_config(p: &PathBuf) -> Result<(), std::io::Error> {
    let mut f = File::create(p)?;
    write!(f, r"[cpu]
cycles = max
core = dynamic

[midi]
mpu401 = none
mididevice = none
")?;
    Ok(())
}
