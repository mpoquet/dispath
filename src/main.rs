use std::env;

use argh::FromArgs;
use anyhow::{Result, bail};
use itertools::Itertools;
use regex::Regex;

#[derive(FromArgs)]
/// Display PATH-like environment variables, one entry per line.
struct MainArgs {
    /// variables to display. default=["PATH"]
    #[argh(positional)]
    vars: Vec<String>,

    /// filter entries to display with a regex. default=".*"
    #[argh(option, short = 'r', default= "String::from(\".*\")")]
    regex: String,

    /// entry separator. default=':'
    #[argh(option, short = 's', default = "':'")]
    sep: char,

    /// do not print the same entry twice (preserves entry order)
    #[argh(switch, short = 'u')]
    unique: bool,

    /// fail if vars are unset
    #[argh(switch)]
    fail_unset: bool,
}

fn main() -> Result<()> {
    let args: MainArgs = argh::from_env();
    let vars = match args.vars.as_slice() {
        [] => vec![String::from("PATH")],
        _ => args.vars,
    };
    let regex = Regex::new(&args.regex)?;
    let mut var_contents = vec![];

    // Read the content of the environment variables
    for var_name in vars.iter() {
        match env::var(var_name) {
            Ok(var_content) => {
                var_contents.push(var_content);
            },
            Err(e) => match e {
                std::env::VarError::NotPresent => if args.fail_unset { bail!("environment variable {var_name} is unset"); },
                _ => bail!("could not read value of environment variable {var_name}: {e}")
            }
        }
    }

    // Print the desired entries
    let entries = var_contents.iter()
        .flat_map(|c| c.split(args.sep))
        .filter(|x| regex.is_match(&x));

    if args.unique {
        for entry in entries.unique() {
            println!("{entry}");
        }
    } else {
        for entry in entries {
            println!("{entry}");
        }
    }

    Ok(())
}

