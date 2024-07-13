use clap::{Arg, Command, value_parser};
use clap_complete::Shell;

pub(crate) fn complete_command() -> Command {
    Command::new("complete")
        .about("Generate shell completion for zsh & bash")
        .arg(
            Arg::new("target")
                .long("shell")
                .value_parser(value_parser!(Shell)),
        )
}

pub(crate) fn yank_command() -> Command {
    Command::new("yank")
        .about("Yank code files from a blueprint")
        .arg(
            Arg::new("blueprint")
                .trailing_var_arg(true),
        )
}