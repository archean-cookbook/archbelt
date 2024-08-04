use clap::{Arg, arg, ArgAction, Command, value_parser};
use clap_complete::Shell;

// TODO: implement clap Parser struct for v0.4.0 w/ full config

pub(crate) fn complete_command() -> Command {
    Command::new("complete")
        .about("Generate shell completion for zsh & bash")
        .arg(
            Arg::new("target")
                .long("shell")
                .value_parser(value_parser!(Shell)),
        )
}

pub(crate) fn info_command() -> Command {
    Command::new("info")
        .about("Display information about the blueprint")
        .arg(blueprint_path_arg())
}

pub(crate) fn package_command() -> Command {
    Command::new("package")
        .about("Gather metadata and files from blueprint for packaging")
        .arg(blueprint_path_arg())
        .arg(
            arg!([BLUEPRINT] "name of the blueprint without .json")
                .num_args(0..)
                .required(true)
                .trailing_var_arg(true))
}

pub(crate) fn yank_command() -> Command {
    Command::new("yank")
        .about("Yank code files from a blueprint")
        .arg(blueprint_path_arg())
        .arg(arg!(-x --"no-collate" "do not collate the files by component")
            .action(ArgAction::SetTrue))
        .arg(arg!(-f --folder "yank blueprint files to folder named after blueprint")
            .action(ArgAction::SetTrue))
        .arg(arg!(-w --watch "watch for changes to the blueprint; yanks files on change; assumes -f")
            .action(ArgAction::SetTrue))
        .arg(
            arg!([BLUEPRINT] "name of the blueprint without .json")
                .num_args(0..)
                .required(true)
                .trailing_var_arg(true))
}

pub(crate) fn watch_command() -> Command {
    Command::new("watch")
        .about("watch for changes to blueprint file(s); will yank on change as if -f is set")
        .arg(blueprint_path_arg())
        .arg(arg!(-x --"no-collate" "do not collate the files by component (use at own risk; this can get very messy..)")
            .action(ArgAction::SetTrue))
        .arg(arg!([TARGET] "target blueprint name (if none given, watches all)")
            .num_args(0..)
            .required(false)
            .trailing_var_arg(true))
}

fn blueprint_path_arg() -> Arg {
    arg!(--"blueprint-path" <PATH> "path to blueprints folder")
        .num_args(1)
        .required(false)
}