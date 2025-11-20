## Graph commands parser

The parser is designed to work with graph commands. It can handle basic operations relating to the vertices and edges of a graph.

In general, it follows this logic:
```
<command> <type> <name> <params>
```

### Parsing logic

Parsing is done through pest crate. Grammar rules are placed in `grammar.pest` file.

Initial parsing rules include:
 
For names:
- `name = { ASCII_ALPHA ~ ASCII_ALPHANUMERIC? }`

For numbers:
- `number = { "-"? ~ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }`

For coordinates of node:
- `coordinates = { "(" ~ number ~ "," ~ number ~ ")" }`

The `grammar.pest` file also contains multiple grammar rules for handling node and edge operations.

For example, delete node rule looks like this:

- `delete_node = { "DELETE" ~ " " ~ "NODE"  ~ " " ~ name ~ (" " ~ name)*}`

At the end I also added rule for parsing file logic:

- `file = { SOI ~ (commands ~ NEWLINE*)+ ~ EOI }`

### Parser functionality

Here are the possible commands and their examples:

| Command  | Example |
| ------------- |:-------------:|
| create node   | `NEW NODE A1 (2,3)`     |
| create edge   | `NEW EDGE A1-B2`     |
| delete node   | `DELETE NODE A1`     |
| delete edge   | `DELETE EDGE A1-B2`     |
| connect path  | `CONNECT PATH A1 -> B2 -> K4 -> L1`     |

Note that we could list multiple objects at one command, for example:

`NEW NODE A1 (2, 3) B1 (4, 4) C1 (-1, 0)`

This parser will help you build a basic graph using commands. It will also help you work with the graph, modify it, and even visualize it.

### Command-line interface (CLI)

Also CLI was implemented for interaction, using clap crate.
The following commands are available:

- `cargo run -- credits ` - get credits information
- `cargo run -- help` - get help
- `cargo run -- read <file.txt>` - parse your custom .txt file. 

I also added two .txt files which could be parsed as an examples:

- input_pass_test.txt  - correct commands
- input_fail_test.txt  - invalid commands 

### Parser usage

The parser can be used to process graph commands. This will help user to directly interact with the created graph, modify it, and define its properties.
