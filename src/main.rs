use std::env;

use argh::FromArgs;
use anyhow::{Result, bail};
use regex::Regex;

#[derive(FromArgs)]
/// Display PATH-like environment variables, one entry per line.
struct MainArgs {
    /// variables to display. default=["PATH"]
    #[argh(positional)]
    vars: Vec<String>,

    /// only matching entries are displayed. default=".*"
    #[argh(option, short = 'r', default= "String::from(\".*\")")]
    regex: String,

    /// entry separator. default=':'
    #[argh(option, short = 's', default = "':'")]
    sep: char,

    /// fail if vars are unset
    #[argh(switch)]
    fail_unset: bool,
}

fn display_path_value(content: String, sep: char, regex: &Regex) {
    for entry in content.split(sep) {
        if regex.is_match(&entry) {
            println!("{entry}");
        }
    }
}

fn main() -> Result<()> {
    let args: MainArgs = argh::from_env();
    let vars = match args.vars.as_slice() {
        [] => vec![String::from("PATH")],
        _ => args.vars,
    };
    let regex = Regex::new(&args.regex)?;

    for var_name in vars.iter() {
        match env::var(var_name) {
            Ok(var_content) => display_path_value(var_content, args.sep, &regex),
            Err(e) => match e {
                std::env::VarError::NotPresent => if args.fail_unset { bail!("environment variable {var_name} is unset"); },
                _ => bail!("could not read value of environment variable {var_name}: {e}")
            }
        }
    }
    Ok(())
}

