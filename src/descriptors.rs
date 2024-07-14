use clap::{Arg, arg, ArgAction, Command, value_parser};
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
        .arg(arg!(-f --folder "yank blueprint files to folder named after blueprint")
            .action(ArgAction::SetTrue))
        .arg(arg!(-w --watch "watch for changes to the blueprint; yanks files on change")
            .action(ArgAction::SetTrue))
        .arg(
            arg!([BLUEPRINT] "name of the blueprint without .json")
                .num_args(0..)
                .required(true)
                .trailing_var_arg(true),
        )
}