use std::env;
use std::path::PathBuf;

extern crate dosbox_lib;

use dosbox_lib::{find_dosbox, DOSBox};

fn main() -> Result<(), String> {
    match find_dosbox() {
        Some(dosbox) => run(dosbox),
        None => Err("Could not find DOSBox".to_string())
    }
}

fn run(dosbox: PathBuf) -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    DOSBox::new()
        .dosbox(dosbox)
        .cwd(&args[0])
        .command(&args[1])
        .run()
}
