# What did I practice?
- Function creation and ways to return values
- `.trim()` and `.parse()`
- Some recursive practice
- Some boolean operations
- Rust Lambda Functions (closures)
- Pattern matching with `match`
- ANSI color codes for terminal output

# What could be improved or made different:

## Improve:
- Make a turn system, that way I don't have to hard code Turn X and O

## Make different:
- The way I compare values to know if there's a winner, some other options: bitwise operation (Johan did it this way), Prime multiplication number, or even Magic Square Sum (up to 15)
- The coloring approach

# Cheat Sheet:

`THING.parse::<DATA_VALUE>()` - Converts a string to a type. If `DATA_VALUE` isn't specified, `u32` is the default.

`let LAMBDA_FUNCTION = |NAME_OF_VALUE: TYPE_OF_VALUE| { /* code */ }` - Creates a "closure" Lambda function.

`format!("{}", NAME_OF_VALUE)` - Returns a String.

`STRING.trim()` - Removes whitespace from the start and end.

`match EXPRESSION { PATTERN => RESULT, _ => DEFAULT }` - Switch the Rust way.

`const NAME: TYPE = VALUE;` - const requires specifying the type of value.

`io::stdin().read_line(&mut VARIABLE)` - adds user input to VARIABLE string.

`print!("\x1Bc")` - ANSI escape code to clear the Terminal.

`"\x1B[38;5;208m"` - ANSI code for setting text color (orange in this case).



