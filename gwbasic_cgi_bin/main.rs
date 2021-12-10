extern crate basic_lib;

use basic_lib::Basic;

fn main() -> Result<(), String> {
    Basic::new()
        .find_basic("GWBASIC.EXE")?
        .bas_program_from_env("PATH_TRANSLATED")?
        .needs_stdin(true)
        .run(|dosbox| dosbox
            .pass_through_env("CONTENT_TYPE")
            .pass_through_env("REQUEST_METHOD"))
}
