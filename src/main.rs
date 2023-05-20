use std::path::PathBuf;

use cubesteak::cubesteak;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = pico_args::Arguments::from_env();
    let Ok(source) = args
        .free_from_os_str(|s| PathBuf::try_from(s))
    else { help() };
    let Ok(target) = args
        .free_from_os_str(|s| PathBuf::try_from(s))
    else { help() };

    cubesteak(source, target)
}

fn help() -> ! {
    eprintln!("Usage: cubesteak <source> <target>");
    std::process::exit(1)
}
