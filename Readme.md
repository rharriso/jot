# Taker 

Note taking app.

## Usage

```
USAGE:
    jot [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>    create note from input file

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    list    list note entries
```

### Commands

* *list* - show notes

# Goals

1. Take input from:
    1. $Editor
    2. stdin if file provided
2. Store locally
    2. Remote sync
3. Note taking format
    1. Markdown with tagging
4. Retrieve by:
    1. Tag
    2. Search
5. List Summary
    1. Date
    2. Tags,
    3. First Few sentences
