use std::env;

extern crate basic_lib;

use basic_lib::Basic;

fn main() -> Result<(), String> {
    Basic::new()
        .find_basic("GWBASIC.EXE")?
        .bas_program_from_arg(env::args().skip(1).next())?
        .run(|dosbox| dosbox)
}
