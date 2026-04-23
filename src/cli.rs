use clap::{Parser, Subcommand};

/// TUI Editor for Music Macro Language (MML)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Check whether a newer build is available on GitHub
    Check,
    /// Start the self-update flow in the background
    Update,
}

pub fn parse_args() -> Args {
    Args::parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_check_subcommand() {
        let args = Args::try_parse_from(["cat-edit-mml", "check"]).unwrap();

        assert_eq!(args.command, Some(Command::Check));
    }

    #[test]
    fn parses_update_subcommand() {
        let args = Args::try_parse_from(["cat-edit-mml", "update"]).unwrap();

        assert_eq!(args.command, Some(Command::Update));
    }

    #[test]
    fn parses_no_subcommand() {
        let args = Args::try_parse_from(["cat-edit-mml"]).unwrap();

        assert_eq!(args.command, None);
    }
}
