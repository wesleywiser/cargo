//use cargo::ops::{dependency, DependencyOptions, AddOptions};
use cargo::ops;
use cargo::core::{MultiShell};
use cargo::util::{CliError, CliResult};

#[deriving(Decodable, Show)]
struct Options {
    arg_name: String,
    flag_git: Option<String>, 
    cmd_add: bool,
    cmd_rm: bool,
}

pub const USAGE: &'static str = "
Manage dependencies for the current project.

Usage:
    cargo dependency add <name> --git <url>
    cargo dependency rm <name>

Options:
    --git <url>    Path to the git repository for the specified dependency.
";

pub fn execute(options : Options, shell : &mut MultiShell) -> CliResult<Option<()>> {
    let ops = 
        if options.cmd_add {
            match options.flag_git {
                Some(url) => 
                    ops::Add(ops::AddOptions {
                        name: options.arg_name,
                        url: url,
                    }),
                None => 
                    return Err(CliError::new("--git is required when adding dependencies", 101))
            }
        //} else if options.arg_rm {
        //    ops::Remove(options.arg_name)
        } else {
            return Err(CliError::new(format!("Command not implemented: {}", options), 101))
        };

    try!(ops::dependency(ops, shell).map_err(|e| {
        CliError::from_boxed(e, 101) //TODO what's the number for?
    }));
    Ok(None)
}
