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

    /// do not print the same entry twice (preserves entry order)
    #[argh(switch, short = 'u')]
    unique: bool,

    /// display all set variables instead of vars
    #[argh(switch, short = 'a')]
    all_vars: bool,

    /// entry separator. default=':'
    #[argh(option, short = 's', default = "':'")]
    sep: char,

    /// fail if vars are unset
    #[argh(switch)]
    fail_unset: bool,
}

fn all_set_variables_values() -> Result<Vec<String>> {
    let mut values = Vec::new();
    for (key, value) in env::vars_os() {
        match value.into_string() {
            Ok(val) => values.push(val),
            _ => match key.into_string() {
                Ok(k) => bail!("could not read value of environment variable {k}"),
                _ => bail!("could not read value of an environment variable whose key is unreadable too... fix your env!"),
            }
        }
    }
    Ok(values)
}

fn variable_values(vars: Vec<String>, fail_unset: bool) -> Result<Vec<String>> {
    let mut values = vec![];
    for var_name in vars.iter() {
        match env::var(var_name) {
            Ok(var_content) => {
                values.push(var_content);
            },
            Err(e) => match e {
                std::env::VarError::NotPresent => if fail_unset { bail!("environment variable {var_name} is unset"); },
                _ => bail!("could not read value of environment variable {var_name}: {e}")
            }
        }
    }
    Ok(values)
}

fn main() -> Result<()> {
    let args: MainArgs = argh::from_env();
    let regex = Regex::new(&args.regex)?;

    // Read the content of the environment variables
    let var_contents = match args.all_vars {
        true => all_set_variables_values()?,
        false => match args.vars.as_slice() {
            [] => variable_values(vec![String::from("PATH")], args.fail_unset)?,
            _ => variable_values(args.vars, args.fail_unset)?
        }
    };

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

