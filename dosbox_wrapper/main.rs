use std::env;

extern crate dosbox_lib;

use dosbox_lib::DOSBox;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    DOSBox::new()
        .find_dosbox()?
        .cwd(&args[0])
        .command(&args[1])
        .run()
}
