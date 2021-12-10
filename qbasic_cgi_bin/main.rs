extern crate basic_lib;

use basic_lib::Basic;

fn main() -> Result<(), String> {
    Basic::new()
        .find_basic("QBASIC.EXE")?
        .bas_program_from_env("PATH_TRANSLATED")?
        .needs_stdin(true)
        .run_arg("/RUN")
        .run(|dosbox| dosbox
            .pass_through_env("CONTENT_TYPE")
            .pass_through_env("QUERY_STRING")
            .pass_through_env("REQUEST_METHOD"))
}
