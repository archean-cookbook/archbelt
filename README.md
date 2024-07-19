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
A tool to work with Archean blueprints and XenonCode modules

Usage: archbelt [COMMAND]

Commands:
  yank      Yank code files from a blueprint
  watch     watch for changes to blueprint file(s); will yank on change
  complete  Generate shell completion for zsh & bash
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Yank
Note, `[BLUEPRINT]...` is the name of the blueprint without the `.json` extension, and no quotes. This will use the exact name of the blueprint saved in-game, letting you Ctrl-C to copy the name just before you save it, then paste it into the command line.

```
Yank code files from a blueprint

Usage: archbelt yank [OPTIONS] [BLUEPRINT]...

Arguments:
  [BLUEPRINT]...  name of the blueprint without .json

Options:
  -x, --no-collate  do not collate the files by component
  -f, --folder      yank blueprint files to folder named after blueprint
  -w, --watch       watch for changes to the blueprint; yanks files on change; assumes -f
  -h, --help        Print help
```

### Watch
Note, `[TARGET]...` is the name of the blueprint without the `.json` extension, and no quotes. This will use the exact name of the blueprint saved in-game, letting you Ctrl-C to copy the name just before you save it, then paste it into the command line.

```
watch for changes to blueprint file(s); will yank on change as if -f is set

Usage: archbelt watch [OPTIONS] [TARGET]...

Arguments:
  [TARGET]...  target blueprint name (if none given, watches all)

Options:
  -x, --no-collate  do not collate the files by component (use at own risk; this can get very messy..)
  -h, --help        Print help
```

### Shell Completion
```
archbelt complete --help
Generate shell completion for zsh & bash

Usage: archbelt complete [OPTIONS]

Options:
      --shell <target>  [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help            Print help
```

## Known Issues
- When creating the files with `yank` or `watch`, `archbelt` will create a folder for each component that has an HDD in the blueprint.  If there is an alias, it will use that for the folder name; if there isn't, it will use the module type's name instead. This means that if there are say, several computers, they will all be saved in the `ARCHEAN_computer` folder. This could lead to files being overwritten.
