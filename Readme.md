# Jot 

Note taking app.

## Usage

```
USAGE:
    jot-down [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>    create note from input file

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    list    list note entries
```

* `jot-down --file /path/to/note/file`
* `jot-down "Add not with this text"`


# List Subcommand  

```
list note entries

USAGE:
    jot-down list [OPTIONS] [QUERY]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --tags <tag>    filter by tags

ARGS:
    <QUERY>    Search the list by string

```

Examples:

* `jot-down list --tags=one,two,three`
* `jot-down list "Search for this text"`

### Commands

* *list* - show notes
