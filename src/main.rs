use std::path::PathBuf;

use cubesteak::cubesteak;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = pico_args::Arguments::from_env();

    if args.contains(["-h", "--help"]) {
        help(0);
    }

    let Ok(source) = args
        .free_from_os_str(|s| PathBuf::try_from(s))
    else { help(1) };
    let Ok(target) = args
        .free_from_os_str(|s| PathBuf::try_from(s))
    else { help(1) };
    if !args.finish().is_empty() {
        help(1);
    }

    cubesteak(source, target)
}

fn help(code: i32) -> ! {
    eprintln!("Usage: cubesteak <source_dir> <target_dir>");
    std::process::exit(code)
}
