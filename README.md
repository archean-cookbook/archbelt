# archbelt

A tool to work with Archean blueprints and XenonCode modules; primarily for use with version control systems and CI/CD.

## Features
The following features are in-development: 

- [ ] Yank code from blueprint as `.xc` file
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

*Coming Soon*

### Shell Completion
```
archbelt complete --help
Generate shell completion for zsh & bash

Usage: archbelt complete [OPTIONS]

Options:
      --shell <target>  [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help            Print help
```