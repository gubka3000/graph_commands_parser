### Graph commands parser

The parser is designed to work with graph commands. 

In general, it has the following idea:
```
<command> <type> <name> <params>
```

So for example creating a new node (vertex) in graph could be done with this:
```
NEW NODE <name> (<x>,<y>)

Ex: NEW NODE F5 (2,-4)
```

As for now, the defined grammar rules are: 
```pest
    // one or more letters or digits
    name = { ASCII_ALPHANUMERIC+ }
    // some digits with possible minus sign 
    number = { "-"? ~ ASCII_DIGIT+ }

    // coordinates for graph, for example: (3,4)
    coordinates = { "(" ~ number ~ "," ~ number ~ ")" }

    // new node command 
    // "NEW NODE A1 (2, 3)"
    new_node = { "NEW" ~ " " ~ "NODE"  ~ " " ~ name  ~ " " ~ coordinates }
```

In the final, the parser should distinguish basic graph commands such as:
- creating objects
- modifying objects
- deleting objects
- analyzing the constructed graph

* By objects we mean vertices and edges

This parser will help you build a basic graph using commands. It will also help you work with the graph, modify it, and even visualize it.

