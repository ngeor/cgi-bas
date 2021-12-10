use std::fs::File;
use std::env;
use std::io::{self, Write};
use std::path::*;

extern crate dosbox_lib;

use dosbox_lib::{find_file_in_path, DOSBox, TempFile};

pub struct Basic {
    basic_exe_filename: String,
    basic_exe_full_path: PathBuf,
    bas_program: PathBuf,
    needs_stdin: bool,
    run_arg: String,
}

impl Basic {
    pub fn new() -> Self {
        Self {
            basic_exe_filename: String::new(),
            basic_exe_full_path: PathBuf::new(),
            bas_program: PathBuf::new(),
            needs_stdin: false,
            run_arg: String::new(),
        }
    }

    pub fn basic_exe(&mut self, basic_exe: PathBuf) -> &mut Self {
        self.basic_exe_filename = basic_exe.file_name().unwrap().to_str().unwrap().to_string();
        self.basic_exe_full_path = basic_exe;
        self
    }

    pub fn find_basic(&mut self, basic_exe_filename: &str) -> Result<&mut Self, String> {
        match find_file_in_path(basic_exe_filename) {
            Some(basic) => Ok(self.basic_exe(basic)),
            None => Err(format!("Could not find {} in PATH", basic_exe_filename))
        }
    }

    pub fn bas_program<T>(&mut self, bas_program: T) -> &mut Self where PathBuf: From<T> {
        self.bas_program = PathBuf::from(bas_program);
        self
    }

    pub fn bas_program_from_arg(&mut self, arg: Option<String>) -> Result<&mut Self, String> {
        match arg {
            Some(x) => {
                if x.is_empty() {
                    Err("Please specify the BAS file to run".to_string())
                } else {
                    Ok(self.bas_program(x))
                }
            }
            _ => Err("Please specify the BAS file to run".to_string())
        }
    }

    pub fn bas_program_from_env(&mut self, env_var_name: &str) -> Result<&mut Self, String> {
        match env::var(env_var_name) {
            Ok(env_var_value) => {
                if env_var_value.is_empty() {
                    Err(format!("Env var {} was empty", env_var_name))
                } else {
                    Ok(self.bas_program(env_var_value))
                }
            }
            Err(_) => Err(format!("Env variable {} was not set", env_var_name))
        }
    }

    pub fn needs_stdin(&mut self, needs_stdin: bool) -> &mut Self {
        self.needs_stdin = needs_stdin;
        self
    }

    pub fn run_arg(&mut self, run_arg: &str) -> &mut Self {
        self.run_arg = run_arg.to_string();
        self
    }

    pub fn run<F>(&mut self, customizer: F) -> Result<(), String> where F : FnMut(&mut DOSBox) -> &mut DOSBox {
        // TODO make bas_program absolute
        if !self.bas_program.is_file() {
            return Err(format!("Could not find BAS file {}", self.bas_program.display()));
        }

        let cwd = self.bas_program.parent().unwrap();

        let basic_exe_copy = TempFile::from(cwd.join(&self.basic_exe_filename));
        copy_without_permissions(&self.basic_exe_full_path, &basic_exe_copy).unwrap();

        // specific to cgi-bin
        let stdin_path : Option<TempFile> = if self.needs_stdin { Some(TempFile::from(cwd.join("STDIN.TXT"))) } else { None };
        if let Some(x) = stdin_path.as_ref() {
            create_stdin(&x).unwrap();
        }

        let cmd = format!("{} {} {}", self.basic_exe_filename, self.run_arg, self.bas_program.file_name().unwrap().to_str().unwrap());

        DOSBox::new()
            .find_dosbox()?
            .cwd(cwd)
            .command(cmd)
            .customize(|dosbox| if stdin_path.is_some() {
                dosbox.env("STDIN", "STDIN.TXT")
            } else {
                dosbox
            })
            .customize(customizer)
            .run()
    }
}

fn copy_without_permissions(src: &Path, dest: &Path) -> Result<u64, io::Error> {
    let mut src_file = File::open(src)?;
    let mut dest_file = File::create(dest)?;
    io::copy(&mut src_file, &mut dest_file)
}

fn create_stdin(stdin_file: &PathBuf) -> io::Result<()> {
    let mut f = File::create(stdin_file)?;
    let stdin = io::stdin();
    loop {
        let mut line = String::new();
        let num_bytes = stdin.read_line(&mut line)?;
        if num_bytes == 0 {
            break;
        }
        write!(f, "{}\r\n", line.trim_end())?;
    }
    Ok(())
}
