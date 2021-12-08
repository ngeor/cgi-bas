use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::*;

extern crate dosbox_lib;

use dosbox_lib::dosbox::DOSBox;
use dosbox_lib::find::{find_dosbox, find_file_in_path};

fn main() -> Result<(), String> {
    match find_dosbox() {
        Some(dosbox) => run(dosbox),
        None => Err("Could not find DOSBox".to_string())
    }
}

fn run(dosbox: PathBuf) -> Result<(), String> {
    match find_file_in_path("GWBASIC.EXE") {
        Some(gwbasic) => run2(dosbox, gwbasic),
        None => Err("Could not find GWBASIC.EXE in PATH".to_string())
    }
}

fn run2(dosbox: PathBuf, gwbasic: PathBuf) -> Result<(), String> {
    match find_bas_file() {
        Ok(bas_file) => run3(dosbox, gwbasic, bas_file),
        Err(err) => Err(err)
    }
}

fn find_bas_file() -> Result<PathBuf, String> {
    match env::var("PATH_TRANSLATED") {
        Ok(path_translated) => {
            if path_translated.is_empty() {
                Err("PATH_TRANSLATED must not be empty".to_string())
            } else {
                let bas_file = PathBuf::from(path_translated);
                if bas_file.is_file() {
                    Ok(bas_file)
                } else {
                    Err(format!("Could not find BAS file {}", bas_file.display()))
                }
            }
        }
        Err(_) => {
            Err("Please specify the BAS file to run".to_string())
        }
    }
}

fn run3(dosbox: PathBuf, gwbasic: PathBuf, bas_file: PathBuf) -> Result<(), String> {
    // copy GWBASIC into the same folder as the BAS_FILE
    let cwd = bas_file.parent().unwrap();
    let stdin_path = cwd.join("STDIN.TXT");
    create_stdin(&stdin_path).unwrap();
    let gwbasic_copy = cwd.join("GWBASIC.EXE");
    copy_without_permissions(&gwbasic, &gwbasic_copy).unwrap();
    let cmd = format!("GWBASIC.EXE {}", bas_file.file_name().unwrap().to_str().unwrap());
    DOSBox::new()
        .dosbox(dosbox)
        .cwd(cwd)
        .command(cmd)
        .pass_through_env("REQUEST_METHOD")
        .pass_through_env("CONTENT_TYPE")
        .env("STDIN", "STDIN.TXT")
        .run().unwrap();
    fs::remove_file(gwbasic_copy).unwrap();
    fs::remove_file(stdin_path).unwrap();
    Ok(())
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