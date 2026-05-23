use anyhow::Result;
use clap::Args;
use colored::Colorize;
use std::env;

#[derive(Args)]
pub struct InfoArgs {
    /// Show environment variables
    #[arg(short, long)]
    pub env: bool,

    /// Show working directory
    #[arg(short, long)]
    pub dir: bool,
}

pub fn run(args: InfoArgs) -> Result<()> {
    println!("{}", "tooler info".bold().cyan());
    println!("{}", "─".repeat(30).dimmed());

    if args.dir || (!args.env && !args.dir) {
        let cwd = env::current_dir()?;
        println!("{} {}", "dir:".bold(), cwd.display().to_string().green());
    }

    if args.env || (!args.env && !args.dir) {
        println!("{}", "env:".bold());
        let important = ["PATH", "HOME", "USER", "SHELL", "TERM"];
        for key in important {
            if let Ok(val) = env::var(key) {
                println!("  {} = {}", key.yellow(), val.dimmed());
            }
        }
    }

    Ok(())
}
