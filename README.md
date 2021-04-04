# bipf-rs

## :japanese_goblin: **[ Work-in-progress ]** :japanese_goblin:

Rust implementation of the Binary In-Place Format.

View the JavaScript implementation [here](https://github.com/ssbc/bipf).

## Data Types

The type indicates the encoding of the value. Valid types are:

```
STRING  : 0  // utf8 encoded string
BUFFER  : 1  // raw binary buffer
INT     : 2  // little endian 32 bit integer
DOUBLE  : 3  // little endian 64 bit float
ARRAY   : 4  // array of any other value
OBJECT  : 5  // list of string: value pairs
BOOLNULL: 6  // a boolean, or null
EXTENDED: 7  // custom type
```

## Tests

Run all tests:

`cargo test`

## License

LGPL 3.0.
