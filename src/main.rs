use exitfailure::ExitFailure;
use failure::ResultExt;
use minigrep::{run, Config};
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let args = Config::from_args();
    println!(
        "pattern: {}, path: {:?}, is_case_sensitive:{:?}",
        &args.pattern, &args.path, &args.is_case_sensitive
    );
    let result: Vec<String> =
        run(&args).with_context(|_| format!("Error running `{:?}`", &args.path))?;
    for line in result {
        println!("{}", line);
    }
    Ok(())
}
