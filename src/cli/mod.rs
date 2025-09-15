use clap::{FromArgMatches, Subcommand};
use color_eyre::Result;
use indoc::indoc;

mod export;
mod fetch;
mod ls;
pub mod version;

pub struct Cli {}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Export(export::Export),
    Fetch(fetch::Fetch),
    Ls(ls::Ls),
    Version(version::Version),
}

impl Commands {
    pub fn run(self) -> Result<()> {
        match self {
            Self::Export(cmd) => cmd.run(),
            Self::Fetch(cmd) => cmd.run(),
            Self::Ls(cmd) => cmd.run(),
            Self::Version(cmd) => cmd.run(),
        }
    }
}

impl Cli {
    pub fn command() -> clap::Command {
        Commands::augment_subcommands(
            clap::Command::new("roast")
                .version(version::VERSION.to_string())
                .about(env!("CARGO_PKG_DESCRIPTION"))
                .author("Roland Schär <@roele>")
                .long_about(LONG_ABOUT)
                .arg_required_else_help(true)
                .subcommand_required(true),
        )
    }

    pub fn run(args: &Vec<String>) -> Result<()> {
        crate::env::ARGS.write().unwrap().clone_from(args);
        version::print_version_if_requested(args)?;

        let matches = Self::command()
            .try_get_matches_from(args)
            .unwrap_or_else(|_| Self::command().get_matches_from(args));

        // debug!("ARGS: {}", &args.join(" "));

        match Commands::from_arg_matches(&matches) {
            Ok(cmd) => cmd.run(),
            Err(err) => matches.subcommand().ok_or(err).map(|_| {
                // No subcommand was provided, so we'll just print the help message
                Self::command().print_help().unwrap();
                Ok(())
            })?,
        }
    }
}

const LONG_ABOUT: &str = indoc! {"
roast a JVM Crawler. https://github.com/jdx/mise-java

A tool to crawl JVM data for the various JVM vendors.
"};
