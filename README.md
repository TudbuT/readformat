# readformat

The inverse of format!().
The format argument is the format string, and the s argument is the string to match the format
against.

Examples:
 - `readf1("Hello, {}!", "Hello, world!")` => `Some("world")`
 - `readf("I hope {} are {}!", "I hope you are doing well!")` => `Some(vec!["you", "well"])`
 - `readf1("Goodbye, {}!", "Hello, world!")` => `None`
