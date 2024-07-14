# archbelt

A tool to work with Archean blueprints and XenonCode modules; primarily for use with version control systems and CI/CD.

## Features
The following features are in-development: 

- [X] Yank code from blueprint as `.xc` file
- [ ] Copy blueprints to current location for packaging
- [ ] Initialize version control
- [X] Shell completion

## Usage
```
archbelt
A tool to work with Archean blueprints and XenonCode modules

Usage: archbelt [COMMAND]

Commands:
  yank      Yank code files from a blueprint
  complete  Generate shell completion for zsh & bash
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help

  -V, --version
          Print version
```

### Yank

Note, `[BLUEPRINT]...` is the name of the blueprint without the `.json` extension, and no quotes. This will use the exact name of the blueprint saved in-game, letting you Ctrl-C to copy the name just before you save it, then paste it into the command line.

```
Yank code files from a blueprint

Usage: archbelt yank [BLUEPRINT]...

Arguments:
  [BLUEPRINT]...  name of the blueprint without .json

Options:
  -h, --help  Print help
```


#### Planned
Later, this will have a -w flag to specify "watch" mode, where it will watch the blueprint directory for changes and automatically yank the files, placing them in folders named after the blueprint(s). This will be useful for collections that update while you're in the game.

### Shell Completion
```
archbelt complete --help
Generate shell completion for zsh & bash

Usage: archbelt complete [OPTIONS]

Options:
      --shell <target>  [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help            Print help
```